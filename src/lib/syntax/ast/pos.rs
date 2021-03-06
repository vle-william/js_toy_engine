#[derive(Clone, PartialEq)]
pub struct Position {
    pub column_number: u64,
    pub line_number: u64,
}

impl Position {
    pub fn new(line_number: u64, column_number: u64) -> Position {
        Position {
            column_number: column_number,
            line_number: line_number,
        }
    }
}
