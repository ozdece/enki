use std::cell::Cell;

#[derive(Debug, PartialEq, Eq)]
enum MarkdownError {
    ExhaustedInput,
}

#[derive(Debug, PartialEq, Eq)]
enum MarkdownToken {
    Header1(String),
    NewLine,
    Text(String),
}

struct MarkdownParser {
    offset: Cell<usize>,
    chars: Vec<char>,
}

impl MarkdownParser {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();

        Self {
            offset: Cell::new(0),
            chars,
        }
    }

    pub fn parse(&self) -> Result<Vec<MarkdownToken>, MarkdownError> {
        let chars_len = self.chars.len();
        let mut result = Vec::with_capacity(10);

        while self.offset.get() < chars_len {
            let ch = self.chars[self.offset.get()];

            match ch {
                '#' => result.push(self.parse_header_or_text()?),
                '\n' => result.push(self.parse_new_line()),
                _ => panic!("Unexpected token {}.", ch),
            }
        }

        Ok(result)
    }

    fn parse_header_or_text(&self) -> Result<MarkdownToken, MarkdownError> {
        // Increase offset so that we can switch to the next character
        self.increase_offset();

        if self.offset.get() == self.chars.len() {
            return Err(MarkdownError::ExhaustedInput);
        }

        let current_ch = self.chars[self.offset.get()];

        if current_ch == ' ' {
            self.increase_offset();
            self.parse_header()
        } else {
            todo!("Text parsing will be done later")
        }
    }

    fn parse_header(&self) -> Result<MarkdownToken, MarkdownError> {
        let mut header_chars = Vec::with_capacity(30);

        while self.offset.get() < self.chars.len() {
            let ch = self.chars[self.offset.get()];

            if ch == '\n' {
                self.offset.set(self.offset.get());
                let header_text: String = header_chars.into_iter().collect();

                return Ok(MarkdownToken::Header1(header_text));
            }

            header_chars.push(ch);
            self.increase_offset();
        }

        let header_text: String = header_chars.into_iter().collect();

        Ok(MarkdownToken::Header1(header_text))
    }

    fn parse_new_line(&self) -> MarkdownToken {
        self.increase_offset();

        MarkdownToken::NewLine
    }

    #[inline]
    fn increase_offset(&self) {
        self.offset.set(self.offset.get() + 1);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_header_1() {
        let input = "# Hello World!";

        let markdown_parser = MarkdownParser::new(input);
        let result = markdown_parser.parse().unwrap();

        assert_eq!(
            result,
            vec![MarkdownToken::Header1("Hello World!".to_string())]
        );
    }

    #[test]
    fn parse_header_1_with_newline() {
        let input = "# Hello World!\n";

        let markdown_parser = MarkdownParser::new(input);
        let result = markdown_parser.parse().unwrap();

        assert_eq!(
            result,
            vec![
                MarkdownToken::Header1("Hello World!".to_string()),
                MarkdownToken::NewLine
            ]
        );
    }
}
