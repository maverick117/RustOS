/* Module for paging */

mod entry;
mod table;
mod temp_page;
mod mapper;

use mem::PAGE_SIZE;
use mem::Frame;

pub use self::entry::*;
use mem::FrameAllocator;
use self::mapper::Mapper;
use core::ops::{Deref, DerefMut};
use self::table::{Table, Level4};
use core::ptr::Unique;
use mem::paging::temp_page::{TemporaryPage};
use multiboot2::BootInformation;

const ENTRY_COUNT : usize = 512;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

extern "C" {
    fn flush_tlb(addr: VirtualAddress);
}

#[derive(Copy,Clone,Debug)]
pub struct Page {
    number : usize,
}

impl Page{
    pub fn containing_address(address: VirtualAddress) -> Page {
        assert!(address < 0x0000_8000_0000_0000 || address > 0xffff_8000_0000_0000, "invalid address: 0x{:x}", address);
        Page{number: address / PAGE_SIZE}
    }

    fn start_address(&self) -> usize {
        self.number * PAGE_SIZE
    }

    fn p4_index(&self) -> usize {
        (self.number >> 27) & 0o777
    }

    fn p3_index(&self) -> usize {
        (self.number >> 18) & 0o777
    }

    fn p2_index(&self) -> usize {
        (self.number >> 9) & 0o777
    }

    fn p1_index(&self) -> usize {
        (self.number >> 0) & 0o777
    }
}

pub struct ActivePageTable {
    mapper: Mapper,
}

impl Deref for ActivePageTable {
    type Target = Mapper;

    fn deref(&self) -> &Mapper {
        &self.mapper
    }
}

impl DerefMut for ActivePageTable {
    fn deref_mut(&mut self) -> &mut Mapper {
        &mut self.mapper
    }
}

impl ActivePageTable {
    pub unsafe fn new() -> ActivePageTable {
        ActivePageTable {
            mapper : Mapper::new()
        }
    }

    pub fn with<F>(&mut self, table: &mut InactivePageTable, temporary_page: &mut TemporaryPage, f: F) where F: FnOnce(&mut ActivePageTable) {
        {
            let backup = Frame::containing_address({let val: usize;unsafe{asm!("mov rax, cr3":"=rax"(val):::"intel","volatile")};val});
            let p4_table = temporary_page.map_table_frame(backup.clone(),self);

            self.p4_mut()[511].set(table.p4_frame.clone(), PRESENT | WRITABLE);

            unsafe{
                asm!("push rax
                      mov rax, cr3
                      mov cr3, rax
                      pop rax"
                      ::::"intel","volatile")
            };

            f(self);

            // Restore original mapping
            p4_table[511].set(backup, PRESENT | WRITABLE);
            unsafe{
                asm!("push rax
                      mov rax, cr3
                      mov cr3, rax
                      pop rax"
                      ::::"intel","volatile")
            };
        }
        temporary_page.unmap(self);
    }


}

pub fn test_paging<A>(allocator: &mut A) where A: FrameAllocator {
    let mut page_table = unsafe{ ActivePageTable::new() };
    let addr = 42*512*512*4096; // 42nd P3 entry
    let page = Page::containing_address(addr);
    let frame = allocator.allocate_frame().expect("no more frames");
    println!("None = {:?}, map to {:?}", page_table.translate(addr), frame);
    page_table.map_to(page,frame,EntryFlags::empty(),allocator);
    println!("Some = {:?}", page_table.translate(addr));
    println!("next free frame: {:?}",allocator.allocate_frame());
    page_table.unmap(Page::containing_address(addr),allocator);
    println!("None = {:?}",page_table.translate(addr));
    //loop{};
    unsafe{flush_tlb(page.start_address() as VirtualAddress)};

    println!("{:#x}",unsafe{*(Page::containing_address(addr).start_address() as *const u64)});


}

pub struct InactivePageTable {
    p4_frame: Frame,
}

impl InactivePageTable {
    pub fn new(frame: Frame, active_table: &mut ActivePageTable, temporary_page : &mut TemporaryPage) -> InactivePageTable {
        // TODO: nullify and recursively map the frame
        {
            let table = temporary_page.map_table_frame(frame.clone(), active_table);
            table.zero();
            table[511].set(frame.clone(), PRESENT | WRITABLE);
        }
        temporary_page.unmap(active_table);

        InactivePageTable { p4_frame: frame }
    }
}

pub fn remap_kernel<A>(allocator: &mut A, boot_info: &BootInformation) where A: FrameAllocator {
    let mut temporary_page = TemporaryPage::new(Page {number: 0xcafebabe}, allocator);
    let mut active_table = unsafe{ ActivePageTable::new() };
    let mut new_table = {
        let frame = allocator.allocate_frame().expect("no more frames");
        InactivePageTable::new(frame, &mut active_table, &mut temporary_page)
    };
    active_table.with(&mut new_table, &mut temporary_page, |mapper| {
        let elf_sections_tag = boot_info.elf_sections_tag().expect("Memory map tag required");
        for section in elf_sections_tag.sections(){
            // TODO mapper.identity_map() all sections
            use self::entry::WRITABLE;

            if !section.is_allocated() {
                continue;
            }

            assert!(section.start_address() % PAGE_SIZE == 0, "Sections need to be page aligned.");

            println!("mapping section at addr: {:#x}, size: {:#x}", section.addr, section.size);

            let flags = WRITABLE;

            let start_frame = Frame::containing_address(section.start_address());
            let end_frame = Frame::containing_address(section.end_address() - 1);
            for frame in Frame::range_inclusive(start_frame, end_frame){
                mapper.identity_map(frame, flags, allocator);
            }
        }
    })
    
}