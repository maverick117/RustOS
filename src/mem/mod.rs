/* Memory module for the kernel */


pub const PAGE_SIZE : usize = 4096;

#[derive(Debug,PartialEq,Eq,PartialOrd,Ord)]
pub struct Frame{
    number: usize,
}

impl Frame {
    fn containing_address(address:usize) -> Frame {
        Frame{number : address / PAGE_SIZE}
    }
}

pub trait FrameAllocator {
    fn allocate(&mut self) -> Option<Frame>;
    fn deallocate(&mut self, frame:Frame);
}

