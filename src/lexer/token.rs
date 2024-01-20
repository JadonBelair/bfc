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

impl From<char> for TokenType {
    fn from(value: char) -> Self {
        match value {
            '+' => Self::Add,
            '-' => Self::Subtract,
            '<' => Self::Left,
            '>' => Self::Right,
            '.' => Self::Output,
            ',' => Self::Input,
            '[' => Self::JumpIfZero,
            ']' => Self::JumpIfNotZero,
            _ => unreachable!(),
        }
    }
}
