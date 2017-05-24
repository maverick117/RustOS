use super::Page;
use super::{ActivePageTable, VirtualAddress};

use memory::Frame;

pub struct TemporaryPage {
    page: Page,
}

impl TemporaryPage {
    pub fn map(&mut self, frame: Frame, active_table: &mut ActivePageTable) -> VirtualAddress {
        use super::entry::WRITABLE;

        assert!(active_table.translate_page(self.page).is_none(), "temp page is already mapped");
        active_table.map_to(self.page, frame, WRITABLE, ???);
        self.page.start_address()
    }
}