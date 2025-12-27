#[derive(Debug, PartialEq, Eq)]
enum MarkdownError {
    ExhaustedInput,
}

#[derive(Debug, PartialEq, Eq)]
enum HeaderLevel {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[derive(Debug, PartialEq, Eq)]
enum TextStyle {
    Normal,
    Italic,
    Bold,
    BoldItalic,
}

#[derive(Debug, PartialEq, Eq)]
enum MarkdownToken {
    Header(HeaderLevel, TextToken),
    NewLine,
}

#[derive(Debug, PartialEq, Eq)]
struct TextToken {
    style: TextStyle,
    text: String,
    children: Vec<TextToken>,
}

struct MarkdownParser {
    offset: usize,
    chars: Vec<char>,
}

impl MarkdownParser {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();

        Self { offset: 0, chars }
    }

    pub fn parse(&mut self) -> Result<Vec<MarkdownToken>, MarkdownError> {
        let chars_len = self.chars.len();
        let mut result = Vec::with_capacity(10);

        while self.offset < chars_len {
            let ch = self.chars[self.offset];

            match ch {
                '#' => result.push(self.parse_header_or_text()?),
                '\n' => result.push(self.parse_new_line()),
                _ => panic!("Unexpected token {}.", ch),
            }
        }

        Ok(result)
    }

    fn parse_header_or_text(&mut self) -> Result<MarkdownToken, MarkdownError> {
        // Increase offset so that we can switch to the next character
        self.offset += 1;

        if self.offset == self.chars.len() {
            return Err(MarkdownError::ExhaustedInput);
        }

        let current_ch = self.chars[self.offset];

        if current_ch == ' ' {
            self.offset += 1;
            self.parse_header()
        } else {
            todo!("Text parsing will be done later")
        }
    }

    fn parse_header(&mut self) -> Result<MarkdownToken, MarkdownError> {
        let mut header_chars = Vec::with_capacity(30);

        while self.offset < self.chars.len() {
            let ch = self.chars[self.offset];

            if ch == '\n' {
                let header_text: String = header_chars.into_iter().collect();

                return Ok(MarkdownToken::Header(
                    HeaderLevel::One,
                    TextToken {
                        style: TextStyle::Normal,
                        text: header_text,
                        children: Vec::new(),
                    },
                ));
            }

            header_chars.push(ch);
            self.offset += 1;
        }

        let header_text: String = header_chars.into_iter().collect();

        Ok(MarkdownToken::Header(
            HeaderLevel::One,
            TextToken {
                style: TextStyle::Normal,
                text: header_text,
                children: Vec::new(),
            },
        ))
    }

    fn parse_new_line(&mut self) -> MarkdownToken {
        self.offset += 1;

        MarkdownToken::NewLine
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_header_1() {
        let input = "# Hello World!";

        let mut markdown_parser = MarkdownParser::new(input);
        let result = markdown_parser.parse().unwrap();

        assert_eq!(
            result,
            vec![MarkdownToken::Header(
                HeaderLevel::One,
                TextToken {
                    style: TextStyle::Normal,
                    text: "Hello World!".to_string(),
                    children: Vec::new()
                }
            )]
        );
    }

    #[test]
    fn parse_header_1_with_newline() {
        let input = "# Hello World!\n";

        let mut markdown_parser = MarkdownParser::new(input);
        let result = markdown_parser.parse().unwrap();

        assert_eq!(
            result,
            vec![
                MarkdownToken::Header(
                    HeaderLevel::One,
                    TextToken {
                        style: TextStyle::Normal,
                        text: "Hello World!".to_string(),
                        children: Vec::new()
                    }
                ),
                MarkdownToken::NewLine
            ]
        );
    }
}
