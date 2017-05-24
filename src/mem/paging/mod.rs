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

    pub fn with<F>(&mut self, table: &mut InactivePageTable, f: F) where F: FnOnce(&mut ActivePageTable) {
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