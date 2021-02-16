//! A Lexer for a little lisp-like language
//!
//! We will create a `Lexer` struct which takes in a string and then
//! iterates over the tokens that it recognizes. The Lexer will capture
//! information such as the type of each token, the location (span) where it came
//! from in the input string, and any add-on data that each token may require.

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct Ident<'a>(&'a str);

#[derive(Debug)]
enum TokenType<'a> {
    LeftParen,
    RightParen,
    Identifier(Ident<'a>),
}

#[derive(Debug)]
struct Token<'a> {
    /// The slice of the input string that this token was parsed from
    source: &'a str,
    /// The type of token that this is
    token: TokenType<'a>,
    /// The starting and ending indices of this token
    span: Span,
}

struct Lexer<'a> {
    /// The input string that we are lexing tokens from
    input: &'a str,
    /// The index into the string that has been lexed so far
    ///
    /// For example, here is where the offset will point at the beginning
    /// of lexing and after lexing each token in the input string:
    ///
    /// ```text
    /// input:     "the quick brown fox jumped over the lazy dog"
    /// offset: 0   ^   ^     ^     ^   ^      ^    ^   ^    ^
    /// offset: 4   ----|     |     |   |      |    |   |    |
    /// offset: 10  ----------|     |   |      |    |   |    |
    /// offset: 16  ----------------|   |      |    |   |    |
    /// offset: 20  --------------------|      |    |   |    |
    /// offset: 27  ---------------------------|    |   |    |
    /// offset: 32  --------------------------------|   |    |
    /// offset: 36  ------------------------------------|    |
    /// offset: 41  -----------------------------------------|
    /// ```
    offset: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Result<Lexer<'a>, String> {
        if !input.is_ascii() {
            return Err("Lexer can only read ascii input".to_string());
        }
        Ok(Lexer {
            input,
            offset: 0,
        })
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if (&self.input[self.offset..]).is_empty() {
            return None;
        }

        // Ignore whitespace characters
        while &self.input[self.offset..self.offset+1] == " " {
            self.offset += 1;
            if self.offset >= self.input.len() { return None; }
        }

        // Easy cases: check if the first character is '(' or ')'
        let ch = &self.input[self.offset..self.offset+1];
        match ch {
            "(" | ")" => {
                let source = &self.input[self.offset..self.offset+1];
                let token = if ch == "(" { TokenType::LeftParen } else { TokenType::RightParen };
                let span = Span { start: self.offset, end: self.offset + 1 };
                self.offset += 1;
                return Some(Token {
                    source,
                    token,
                    span,
                })
            }
            // Anything else needs to be collected as an identifier
            _other => (),
        }

        // We are looking for an identifier, iterate to the end of it
        let mut end_offset = self.offset;
        let slice = &self.input[self.offset..];
        for ch in slice.chars() {
            if !ch.is_alphabetic() {
                break;
            }
            end_offset += 1;
        }

        let source = &self.input[self.offset..end_offset];
        let ident = Ident(source);
        let token = Token {
            source,
            token: TokenType::Identifier(ident),
            span: Span { start: self.offset, end: end_offset }
        };

        self.offset = end_offset + 1;
        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = "  ( one two )  ";
        println!("String: \"{}\"", input);
        let lexer = Lexer::new(input).unwrap();

        for token in lexer {
            println!("{:?}", token);
        }
    }
}
