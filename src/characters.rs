use std::clone::Clone;

pub struct ParserData {
    input: String,
    output: Vec<String>,
    offset: usize
}

err_type!(pub, OutOfInputError, "No input remains despite requiring input");

impl ParserData {
    pub fn new(input: &String) -> ParserData {
        ParserData { input: input.clone(), output: Vec::new(), offset: 0 }
    }

    pub fn eat(&mut self) -> Result<char, OutOfInputError> {
        let result = self.peek()?;
        self.offset += 1;
        Ok(result)
    }

    pub fn eat_and_push(&mut self) -> Result<(), OutOfInputError> {
        let result = self.eat();
        self.push(result?);
        Ok(())
    }

    pub fn peek(&self) -> Result<char, OutOfInputError> {
        self.input.chars().nth(self.offset).ok_or(OutOfInputError)
    }

    pub fn not_empty(&self) -> bool {
        self.offset < self.input.len()
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