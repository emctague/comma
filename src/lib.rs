use std::str::FromStr;
use std::iter::FromIterator;
use core::fmt;
use std::fmt::Formatter;
use std::error;

/// Contains the result of a parsed command. See [from_str] documentation for details on available
/// command syntax.
#[derive(Debug)]
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
    /// this case it will provide an [EmptyCommandError].
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        // List of all command tokens while splitting the string
        let mut tokens: Vec<String> = Vec::new();
        // True if the previous character was whitespace - used to collapse whitespace
        let mut prev_whitespace = true;
        // True if currently inside of quotes
        let mut in_quotes = false;
        // True if previous character was a backslash and this character should be escaped
        let mut in_backslash = false;

        // The loop assumes the latest item in `tokens` is the current argument, so one must exist
        tokens.push(String::from(""));

        // Iterate all characters and interpret them
        for c in s.chars() {

            // If currently in a backslash, add the next character verbatim no matter what
            if in_backslash {
                prev_whitespace = false;
                in_backslash = false;
                tokens.last_mut().unwrap().push(c);

            // Handle the current character being a backslash
            } else if c == '\\' {
                in_backslash = true;

            // If currently quoted, copy verbatim except for end quotes
            } else if in_quotes {
                prev_whitespace = false;
                if c == '\"' {
                    in_quotes = false;
                } else {
                    tokens.last_mut().unwrap().push(c);
                }

            // If whitespace, add a new empty token unless one has already been added during this
            // run of whitespace
            } else if c.is_whitespace() {
                if !prev_whitespace {
                    tokens.push(String::from(""));
                }
                prev_whitespace = true;

            // For all further conditions we can strike out the whitespace state
            } else {
                prev_whitespace = false;

                // Enter quotation mode if a quote is present
                if c == '\"' {
                    in_quotes = true;

                // Just copy the character verbatim
                } else {
                    tokens.last_mut().unwrap().push(c);
                }
            }
        }

        // Prevents whitespace at the end of the command from creating an empty garbage argument.
        if tokens.last().unwrap().is_empty() {
            tokens.pop();
        }

        if tokens.is_empty() {
            // Fail if no command was provided
            Err(EmptyCommandError)
        } else {
            // Turn the first token into the command name and others into arguments
            Ok(Command { name: tokens[0].clone(), arguments: Vec::from_iter(tokens[1..].iter().cloned()) })
        }
    }
}



/// This error is given by [Command::from_str] when the command string provided is entirely empty.
#[derive(Debug, Clone)]
pub struct EmptyCommandError;

impl fmt::Display for EmptyCommandError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "command string has no command name or arguments")
    }
}

impl error::Error for EmptyCommandError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
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
        let result = Command::from_str("hello world \\\"this is\\\" a \"quoted \\\"string\\\"\"").unwrap();
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
