use crate::characters::ParserData;

/// A `SyntaxBlock` is any syntactical element present in a command string.
pub trait SyntaxBlock {
    /// `consume` checks if the available input matches this type of syntax block, and if it does,
    /// 'eats' up the portion matching the block. It returns true if this match occurred, and false
    /// if it did not.
    fn consume(&self, input: &mut ParserData) -> bool;
}

/// `handle_blocks` iterates over a set of SyntaxBlock objects, attempting to consume data from each
/// one in the given order. If any match, it will stop iterating and return true. If no matches are
/// found, it returns false and does not eat any input.
///
/// This behavior is used to check if any special syntax blocks can be used at the moment.
pub fn handle_blocks(input: &mut ParserData, types: &Vec<&dyn SyntaxBlock>) -> bool {
    if input.not_empty() {
        for t in types {
            if t.consume(input) { return true; }
        }
    }

    false
}

/// `handle_or_push` tests if the given SyntaxBlocks are able to consume available input, in order,
/// stopping when a block successfully eats one or more characters. If no blocks eat characters,
/// `handle_or_push` will instead eat the first available character and push it to the output.
///
/// This behavior is used to handle any nested syntax blocks where plaintext should be pushed.
pub fn handle_or_push(input: &mut ParserData, types: &Vec<&dyn SyntaxBlock>) {
    if !handle_blocks(input, types) {
        input.eat_and_push().unwrap();
    }
}


/// `EscapeBlock` handles a single character prefixed by a backslash, copying this character
/// verbatim to the output.
pub struct EscapeBlock;

impl SyntaxBlock for EscapeBlock {
    fn consume(&self, input: &mut ParserData) -> bool {
        if input.peek().unwrap() == '\\' {
            input.eat().unwrap();
            input.eat_and_push().unwrap_or_default();
            true
        } else {
            false
        }
    }
}

/// `WhitespaceBlock` eats up consecutive whitespace, creating a new token in the process.
pub struct WhitespaceBlock;

impl SyntaxBlock for WhitespaceBlock {
    fn consume(&self, input: &mut ParserData) -> bool {
        if !input.peek().unwrap().is_whitespace() { false }
        else {
            input.new_token();
            while input.not_empty() && input.peek().unwrap().is_whitespace() {
                input.eat().unwrap();
            }
            true
        }
    }
}

/// `QuoteBlock` handles text between quotation marks, where whitespace can be ignored.
/// `EscapeBlock` is valid inside a `QuoteBlock`.
pub struct QuoteBlock;

impl SyntaxBlock for QuoteBlock {
    fn consume(&self, input: &mut ParserData) -> bool {
        if input.peek().unwrap() == '"' {
            input.eat().unwrap();

            while input.not_empty() && input.peek().unwrap() != '"' {
                handle_or_push(input, &vec![ &EscapeBlock{} ]);
            }

            input.eat().unwrap_or_default();

            true
        } else {
            false
        }
    }
}