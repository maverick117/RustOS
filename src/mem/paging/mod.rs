/* Module for paging */

mod entry;
mod table;
mod temp_page;
mod mapper;

use mem::PAGE_SIZE;
use mem::Frame;

pub use self::entry::*;
use mem::FrameAllocator;
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
    p4 : Unique<Table<Level4>>,
}

impl ActivePageTable {
    pub unsafe fn new() -> ActivePageTable {
        ActivePageTable {
            p4: Unique::new(table::P4),
        }
    }

    fn p4(&self) -> &Table<Level4> {
        unsafe { self.p4.as_ref() }
    }

    fn p4_mut(&mut self) -> &mut Table<Level4> {
        unsafe { self.p4.as_mut() }
    }

    pub fn translate(&self, virtual_address : VirtualAddress) -> Option<PhysicalAddress> {
        let offset = virtual_address % PAGE_SIZE;
        self.translate_page(Page::containing_address(virtual_address)).map(|frame| frame.number * PAGE_SIZE + offset)
    }

    fn translate_page(&self, page: Page) -> Option<Frame> {
        use self::entry::HUGE_PAGE;

        //let p3 = unsafe {&*table::P4}.next_table(page.p4_index());
        let p3 = self.p4().next_table(page.p4_index());
        let huge_page = || {
            p3.and_then(|p3| {
                let p3_entry = &p3[page.p3_index()];
                // See if it's a 1GiB page
                if let Some(start_frame) = p3_entry.pointed_frame() {
                    if p3_entry.flags().contains(HUGE_PAGE) {
                        assert!(start_frame.number % (ENTRY_COUNT * ENTRY_COUNT) == 0);
                        return Some(Frame {
                            number: start_frame.number + page.p2_index() * ENTRY_COUNT + page.p1_index(),
                        });
                    }
                }
                if let Some(p2) = p3.next_table(page.p3_index()) {
                    let p2_entry = &p2[page.p2_index()];
                    // See if it's a 2MiB page
                    if let Some(start_frame) = p2_entry.pointed_frame() {
                        if p2_entry.flags().contains(HUGE_PAGE) {
                            assert!(start_frame.number % ENTRY_COUNT == 0);
                            return Some(Frame {
                                number: start_frame.number + page.p1_index()
                            });
                        }   
                    }
                }
                None
            })
        };

        p3.and_then(|p3| p3.next_table(page.p3_index()))
          .and_then(|p2| p2.next_table(page.p2_index()))
          .and_then(|p1| p1[page.p1_index()].pointed_frame())
          .or_else(huge_page)
    }

    pub fn map_to<A>(&mut self, page : Page, frame: Frame, flags: EntryFlags, allocator: &mut A) where A: FrameAllocator {
        let mut p3 = self.p4_mut().next_table_create(page.p4_index(), allocator);
        let mut p2 = p3.next_table_create(page.p3_index(), allocator);
        let mut p1 = p2.next_table_create(page.p2_index(), allocator);

        assert!(p1[page.p1_index()].is_unused());
        p1[page.p1_index()].set(frame, flags| PRESENT);
    }

    pub fn map<A>(&mut self, page:Page, flags: EntryFlags, allocator: &mut A) where A: FrameAllocator {
        let frame = allocator.allocate_frame().expect("Out of memory");
        self.map_to(page,frame,flags,allocator);
    }

    pub fn identity_map<A>(&mut self, frame:Frame, flags: EntryFlags, allocator: &mut A) where A: FrameAllocator {
        let page = Page::containing_address(frame.start_address());
        self.map_to(page,frame,flags,allocator);
    }

    fn unmap<A>(&mut self, page: Page, allocator: &mut A) where A: FrameAllocator {
        assert!(self.translate(page.start_address()).is_some());

        let p1 = self.p4_mut().next_table_mut(page.p4_index())
            .and_then(|p3| p3.next_table_mut(page.p3_index()))
            .and_then(|p2| p2.next_table_mut(page.p2_index()))
            .expect("mapping code does not support huge pages!");
        let frame = p1[page.p1_index()].pointed_frame().unwrap();
        p1[page.p1_index()].set_unused();
        // TODO deallocate tables if empty
        allocator.deallocate_frame(frame);
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