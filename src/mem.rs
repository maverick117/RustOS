/* Memory module for the kernel */

pub struct Page{
    address : u64,
}

pub trait Allocator {
    fn allocate() -> Result<Page, PageErrors>;
    fn deallocate(page: Page) -> Option<PageErrors>;
}

pub enum PageErrors{
    PhysicalMemFull,
}