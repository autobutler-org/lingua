// 1-based line and Unicode scalar column
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}
