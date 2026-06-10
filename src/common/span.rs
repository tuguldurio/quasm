use std::fmt;
use std::ops::Range;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub line: u32,
    pub column: u32
}

// end is exclusive
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: Pos,
    pub end: Pos
}

impl Span {
    pub fn to(self, other: Span) -> Span {
        Span { start: self.start, end: other.end }
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start.line == self.end.line {
            write!(f, "{}:{}-{}", self.start.line, self.start.column, self.end.column)
        } else {
            write!(f, "{}-{}", self.start, self.end)
        }
    }
}

// Compact one-line form so {:#?} AST dumps stay readable
impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

// Converts byte offsets into line/column positions
pub struct LineMap {
    line_starts: Vec<usize>
}

impl LineMap {
    pub fn new(src: &str) -> Self {
        let mut line_starts = vec![0];
        for (i, b) in src.bytes().enumerate() {
            if b == b'\n' {
                line_starts.push(i + 1);
            }
        }
        Self { line_starts }
    }

    pub fn pos(&self, byte: usize) -> Pos {
        let line = self.line_starts.partition_point(|&start| start <= byte) - 1;
        Pos {
            line: (line + 1) as u32,
            column: (byte - self.line_starts[line] + 1) as u32
        }
    }

    pub fn span(&self, range: Range<usize>) -> Span {
        Span { start: self.pos(range.start), end: self.pos(range.end) }
    }
}
