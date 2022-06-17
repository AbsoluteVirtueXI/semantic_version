use crate::token::Token;

/// Semantic version scanner
struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    column: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: String::from(source),
            tokens: vec![],
            start: 0,
            current: 0,
            column: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        self.scan_token();
        unimplemented!()
    }

    fn scan_token(&mut self) -> Result<(), String> {
        unimplemented!()
    }
}
