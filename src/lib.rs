//! `comma` parses command-line-style strings. See [`parse_command`] for details.

use std::iter::Peekable;
use std::str::Chars;

fn parse_escape(ch: char, chars: &mut Peekable<Chars>) -> Option<char> {
    Some(match ch {
        '\\' => match chars.next()? {
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            literal => literal,
        },
        x => x
    })
}

fn parse_string(chars: &mut Peekable<Chars>, delim: char) -> Option<String> {
    let mut output = String::new();

    while let Some(ch) = chars.next() {
        if ch == delim { return Some(output) }
        output.push(parse_escape(ch, chars)?);
    }

    None
}

/// Parses a command into a list of individual tokens.
/// Each token is separated by one or more characters of whitespace.
/// Pairs of single- or double-quotes can be used to ignore whitespace. Within pairs of quotation
/// marks, a backslash (\) can be used to escape any character. The special escape sequences
/// '\n', '\r', and '\t' are also handled as Newlines, Carriage Returns, and Tabs, respectively.
/// Should a quotation mark be mismatched (no counterpart terminating mark exists), this function
/// will return None. Otherwise, it returns a list of tokens in the input string.
pub fn parse_command(input: &str) -> Option<Vec<String>> {
    let mut next_push = true;
    let mut chars = input.chars().peekable();
    let mut output: Vec<String> = Vec::new();

    while let Some(ch) = chars.next() {
        if ch.is_whitespace() {
            next_push = true;
            continue;
        }

        if next_push {
            output.push(String::new());
            next_push = false;
        }

        match ch {
            '"' | '\'' =>
                output
                .last_mut()?
                .push_str(parse_string(&mut chars, ch)?.as_str()),
            ch => output.last_mut()?.push(parse_escape(ch, &mut chars)?)
        }
    }

    Some(output)
}

#[cfg(test)]
mod tests {
    use crate::parse_command;

    #[test]
    fn parsing_works() {
        let result =
            parse_command("   hello    world \\'this is\\' a \"quoted \\\"string\\\"\"    ").unwrap();
        assert_eq!(
            result,
            vec![
                String::from("hello"),
                String::from("world"),
                String::from("'this"),
                String::from("is'"),
                String::from("a"),
                String::from("quoted \"string\"")
            ]
        );
    }

    #[test]
    fn fail_mismatch() {
        assert_eq!(parse_command("Hello 'world "), None);
    }

    #[test]
    fn unicode() {
        // This contains a CJK IDEOGRAPH EXTENSION G character, which is invisible.
        let result = parse_command("ß 𱁬").unwrap();
        assert_eq!(result, vec![String::from("ß"), String::from("𱁬")]);
    }
}
