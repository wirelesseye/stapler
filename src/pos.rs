#[derive(Debug, Clone, Copy)]
pub struct CursorPos {
    row: usize,
    col: usize,
}

impl CursorPos {
    pub fn of(row: usize, col: usize) -> Self {
        Self {
            row,
            col
        }
    }
}