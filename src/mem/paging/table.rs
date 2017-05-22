
use core::ops::{Index, IndexMut};

use mem::paging::entry::*;
use mem::paging::ENTRY_COUNT;

pub struct Table {
    entries : [Entry; ENTRY_COUNT],
}

impl Index<usize> for Table {
    type Output = Entry;

    fn index(&self, index:usize) -> &Entry {
        &self.entries[index]
    }
}

impl IndexMut<usize> for Table {
    fn index_mut(&mut self, index:usize) -> &mut Entry {
        &mut self.entries[index]
    }
}

impl Table {
    pub fn zero(&mut self){
        for entry in self.entries.iter_mut() {
            entry.set_unused();
        }
    }
}