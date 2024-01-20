#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub amount: usize,
}

#[derive(Debug)]
pub enum TokenType {
    Add,
    Subtract,
    Left,
    Right,
    Output,
    Input,
    JumpIfZero,
    JumpIfNotZero,
}
