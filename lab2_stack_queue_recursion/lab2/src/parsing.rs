use std::io::{self, BufRead};

#[derive(Debug)]
pub enum ParseError {
    Io(io::Error),
    // We have some certain expections for what's coming, which doesn't match what we get.
    Expected(&'static str),
    // The number you entered is above the maximum of a type (u64, u32, u8 or usize).
    NumberTooLarge,
    // For some reason, there is unexpected information in the event for end.
    InvalidEnd,
    // The end is missing.
    MissingEnd,
}

impl From<io::Error> for ParseError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Io(error) => write!(f, "I/O error: {error}"),
            ParseError::Expected(what) => write!(f, "expected {what}"),
            ParseError::NumberTooLarge => write!(f, "number is too large"),
            ParseError::InvalidEnd => write!(f, "the end record (E, 0, 0) has unexpected values"),
            ParseError::MissingEnd => write!(f, "missing the end record (E, 0, 0)"),
        }
    }
}

pub struct Parser<R: BufRead> {
    reader: R,
    line: String,
    pos: usize,
}

impl<R: BufRead> Parser<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            line: String::new(),
            pos: 0,
        }
    }

    // Look at the next character without consuming it.
    pub fn peek(&mut self) -> Result<Option<char>, ParseError> {
        // It combines two things:
        // 1. Check whether the current buffer has been exhausted.
        // 2. Keep refilling the buffer until it contains data or EOF is reached.
        while self.pos == self.line.len() {
            self.line.clear();
            self.pos = 0;

            // An empty line we think of like "\n" actually isn't empty.
            // Here, we are able to use the ? operator following read_line()
            // is due to our implementation of From trait.
            // read_line returns io::Error, which is an alias for Result<T, io::Error>
            if self.reader.read_line(&mut self.line)? == 0 {
                return Ok(None);
            }
        }

        // [Note]
        // There are a lot to take in from this line.
        //
        // Rust deliberately does not support indexing a `String` by an integer,
        // because UTF-8 characters have variable byte lengths.
        //
        // So instead, we take a slice beginning at the byte offset `pos`
        // To correctly interpret those UTF-8 bytes as Unicode characters, we use iterators then.
        //
        // It returns an owned `char` value.
        // But that's not directly related to the `Copy` trait,
        // instead strings don't store `char` values, and it stores UTF-8 bytes!!
        // So it doesn't need to copy a `char` out of the original string.
        Ok(self.line[self.pos..].chars().next())
    }

    // Consume exactly one character.
    fn take(&mut self) -> Result<Option<char>, ParseError> {
        let ch = self.peek()?;

        if let Some(ch) = ch {
            self.pos += ch.len_utf8();
        }

        Ok(ch)
    }

    // Consume all consecutive whitespace.
    pub fn skip_ws(&mut self) -> Result<(), ParseError> {
        while self.peek()?.is_some_and(char::is_whitespace) {
            self.take()?;
        }

        Ok(())
    }

    // Consume the next non-whitespace character.
    pub fn next(&mut self) -> Result<Option<char>, ParseError> {
        self.skip_ws()?;
        self.take()
    }

    // Expect one character from a set of acceptable characters.
    // If the expectation is met, we don't additionally return it, but we consume it.
    // but if the expectation is not met, we return an error.
    pub fn expect(
        &mut self,
        choices: &[char],
        name: &'static str,
    ) -> Result<(), ParseError> {
        self.skip_ws()?;

        match self.peek()? {
            Some(ch) if choices.contains(&ch) => {
                self.take()?;
                Ok(())
            }
            _ => Err(ParseError::Expected(name)),
        }
    }

    // Consume the rest of the current line (including the trailing newline).
    //
    // This is useful after a failed parse during interactive input, so that a bad
    // attempt doesn't leave trailing garbage behind to be re-read on the next try.
    pub fn discard_line(&mut self) -> Result<(), ParseError> {
        loop {
            match self.take()? {
                Some('\n') | None => return Ok(()),
                Some(_) => {}
            }
        }
    }

    pub fn parse_u64(&mut self) -> Result<u64, ParseError> {
        self.skip_ws()?;

        let mut value = 0u64;
        let mut found = false;

        while let Some(ch) = self.peek()? {
            if !ch.is_ascii_digit() {
                break;
            }

            found = true;

            value = value
                // return `None` if overflow occurs, so there's a cap of the maximum value we can store here.
                .checked_mul(10)
                // since u8 can only represent 0 ~ 255, this conversion is lossy
                // and the higher bits are discarded, but it's irrelevant here and thus still safe.
                .and_then(|n| n.checked_add((ch as u8 - b'0') as u64))
                .ok_or(ParseError::NumberTooLarge)?;

            self.take()?;
        }

        if found {
            Ok(value)
        } else {
            Err(ParseError::Expected("unsigned integer"))
        }
    }

    pub fn parse_u32(&mut self) -> Result<u32, ParseError> {
        // since the logic for parsing numbers are essentially the same.
        // we don't need multiple implementations for parsing a number just because their sizes are different.
        u32::try_from(self.parse_u64()?).map_err(|_| ParseError::NumberTooLarge)
    }

    pub fn parse_u8(&mut self) -> Result<u8, ParseError> {
        // choices are small, so a byte-wide integer is more than enough.
        u8::try_from(self.parse_u64()?).map_err(|_| ParseError::NumberTooLarge)
    }

    pub fn parse_usize(&mut self) -> Result<usize, ParseError> {
        usize::try_from(self.parse_u64()?).map_err(|_| ParseError::NumberTooLarge)
    }
}
