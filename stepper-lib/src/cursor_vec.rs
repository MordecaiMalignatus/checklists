#[derive(Debug)]
pub struct CursorVec<T> {
    pub items: Vec<T>,
    pub cursor: usize,
}

impl<T> CursorVec<T> {
    pub fn with_items(entries: Vec<T>) -> Self {
        Self {
            items: entries,
            cursor: 0,
        }
    }

    pub fn next(&mut self) {
        if self.cursor < (self.items.len() - 1) {
            self.cursor += 1
        }
    }

    pub fn previous(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1
        }
    }

    pub fn current(&self) -> &T {
        self.items
            .get(self.cursor)
            .expect("Current Element must be in bounds")
    }
}

impl<T> From<Vec<T>> for CursorVec<T> {
    fn from(items: Vec<T>) -> Self {
        Self { items, cursor: 0 }
    }
}
