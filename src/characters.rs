use std::clone::Clone;

pub struct ParserData {
    input: String,
    output: Vec<String>,
    byte_offset: usize,
}

err_type!(
    pub,
    OutOfInputError,
    "No input remains despite requiring input"
);

impl ParserData {
    pub fn new(input: &String) -> ParserData {
        ParserData {
            input: input.clone(),
            output: Vec::new(),
            byte_offset: 0,
        }
    }

    pub fn eat(&mut self) -> Result<char, OutOfInputError> {
        let result = self.peek()?;
        self.byte_offset += result.len_utf8();
        Ok(result)
    }

    pub fn eat_and_push(&mut self) -> Result<(), OutOfInputError> {
        let result = self.eat();
        self.push(result?);
        Ok(())
    }

    pub fn peek(&self) -> Result<char, OutOfInputError> {
        self.input[self.byte_offset..]
            .chars()
            .next()
            .ok_or(OutOfInputError)
    }

    pub fn not_empty(&self) -> bool {
        self.byte_offset < self.input.len()
    }

    pub fn new_token(&mut self) {
        self.output.push(String::new());
    }

    pub fn push(&mut self, c: char) {
        self.output.last_mut().unwrap().push(c);
    }

    pub fn get_result(&self) -> &Vec<String> {
        &self.output
    }
}
