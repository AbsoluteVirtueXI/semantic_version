// TODO: Bad token kind list,
#[derive(Debug)]
enum TokenKind {
    Major(u64),
    Minor(u64),
    Patch(u64),
    PreRelease(Option<String>),
    build(Option<String>),
    Dot,
    Hyphen,
    Plus,
    NumericIdentifier,
    AlphaNumericIdentifier,
}

struct Token {
    kind: TokenKind,
    position: usize,
    lexeme: String,
    // TODO: more field to add.
}
