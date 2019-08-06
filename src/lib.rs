//! `comma` parses command-line-style strings. See [`Command::from_str`] for syntax details,
//! and [`Command`] for structure details.

use std::str::FromStr;
use std::iter::FromIterator;
use characters::ParserData;
use syntax_blocks::*;

#[macro_use]
mod error_types;
mod syntax_blocks;
mod characters;

err_type!(pub, EmptyCommandError, "command string has no command name or arguments");

/// Contains the result of a parsed command. See [`Command::from_str`] documentation for details on
/// available command syntax.
#[derive(Debug, Clone)]
pub struct Command {
    /// The name of the command being run (i.e. the first argument)
    pub name: String,
    /// All arguments being passed
    pub arguments: Vec<String>
}

impl FromStr for Command {
    type Err = EmptyCommandError;

    /// Parse a command from the commandline. Commands consist of separate 'tokens' separated by
    /// whitespace.
    ///
    /// Multiple whitespace characters are permitted between tokens, including at the beginning and
    /// end of the command string. All extra whitespace is stripped unless explicitly escaped using
    /// quotation marks or backslash escaping.
    ///
    /// Preceding a character with a backslash (`\`) will cause any special meaning for the
    /// character to be ignored. To convey a *real* backslash in a command it must be prefixed with
    /// another backslash, such as: (`\\`).
    ///
    /// Quotation marks surrounding a portion of text will also cause the text to be included
    /// verbatim, including whitespace. However, backslashes retain their special meaning, to allow
    /// for escaped quotes (`\"`) inside a quoted string.
    ///
    /// `from_str` will only fail if zero tokens are provided (i.e. there is no command name). In
    /// this case it will provide an [`EmptyCommandError`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // We prepend whitespace to force the whitespace syntax block to add in a token.
        let mut input = String::from(" ");
        input.push_str(s);

        // Parse all data using syntax blocks
        let mut data = ParserData::new(&input);
        while data.not_empty() {
            handle_or_push(&mut data, &vec![ &EscapeBlock{}, &QuoteBlock{}, &WhitespaceBlock{} ]);
        }
        let mut tokens = data.get_result().clone();

        // Prevents whitespace at the end of the command from creating an empty garbage argument.
        if tokens.last().unwrap().is_empty() {
            tokens.pop();
        }

        if tokens.is_empty() {
            // Fail if no command was provided
            Err(EmptyCommandError)
        } else {
            // Turn the first token into the command name and others into arguments
            Ok(Command {
                name: tokens[0].clone(),
                arguments: Vec::from_iter(tokens[1..].iter().cloned())
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Command;
    use std::str::FromStr;

    #[test]
    fn no_arguments_works() {
        let result = Command::from_str("hello").unwrap();
        if !result.name.eq(&String::from("hello")) {
            panic!("Argument-free command doesn't handle command name correctly");
        }
        if !result.arguments.is_empty() {
            panic!("Argument-free command doesn't have empty argument list");
        }
    }

    #[test]
    fn arguments_works() {
        let result =
            Command::from_str("hello world \\\"this is\\\" a \"quoted \\\"string\\\"\"")
                .unwrap();
        if !result.arguments.len() == 5 {
            panic!("Wrong number of arguments parsed");
        }
        if !result.arguments[1].eq(&String::from("\"this")) {
            panic!("Escaped quotes not handled correctly");
        }
        if !result.arguments[4].eq(&String::from("quoted \"string\"")) {
            panic!("Quoted string not handled correctly");
        }
    }

    #[test]
    #[should_panic]
    fn empty_fails() {
        Command::from_str("    ").unwrap();
    }
}
