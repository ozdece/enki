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
enum MarkdownToken {
    Header(HeaderLevel, Vec<TextToken>),
    NewLine,
    Paragraph(Vec<TextToken>),
}

#[derive(Debug, PartialEq, Eq)]
enum TextToken {
    Text(String),
    Italic(Vec<TextToken>),
    Bold(Vec<TextToken>),
    BoldItalic(Vec<TextToken>),
    Code(Vec<TextToken>),
}

struct MarkdownParser {
    offset: usize,
    chars: Vec<char>,
    chars_len: usize,
    stack: Vec<String>,
}

impl MarkdownParser {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();

        Self {
            offset: 0,
            chars_len: chars.len(),
            chars,
            stack: Vec::with_capacity(5),
        }
    }

    pub fn parse(&mut self) -> Vec<MarkdownToken> {
        let chars_len = self.chars.len();
        let mut result = Vec::with_capacity(10);

        while self.offset < chars_len {
            let ch = self.chars[self.offset];

            match ch {
                '#' => result.push(self.parse_header_or_text()),
                '\n' => result.push(self.parse_new_line()),
                _ => panic!("Unexpected token {}.", ch),
            }
        }

        eprintln!("Stack: {:?}", self.stack);

        result
    }

    fn parse_header_or_text(&mut self) -> MarkdownToken {
        // Figure out the level of the header (should be between 1 and 6)
        let mut header_ch_count = 0;
        while self.offset < self.chars_len && self.chars[self.offset] == '#' {
            header_ch_count += 1;
            self.offset += 1;
        }

        if self.offset == self.chars_len {
            return MarkdownToken::Paragraph(vec![]);
        }

        let current_ch = self.chars[self.offset];

        if current_ch == ' ' {
            self.offset += 1;

            let header_level = match header_ch_count {
                1 => HeaderLevel::One,
                2 => HeaderLevel::Two,
                3 => HeaderLevel::Three,
                4 => HeaderLevel::Four,
                5 => HeaderLevel::Five,
                6 => HeaderLevel::Six,
                _ => HeaderLevel::Six,
            };

            self.parse_header(header_level)
        } else {
            todo!("Text parsing will be done later")
        }
    }

    fn parse_header(&mut self, header_level: HeaderLevel) -> MarkdownToken {
        let text_tokens = self.parse_text_tokens();

        MarkdownToken::Header(header_level, text_tokens)
    }

    fn parse_text_tokens(&mut self) -> Vec<TextToken> {
        if self.offset == self.chars_len {
            return vec![];
        }

        let mut tokens = Vec::with_capacity(10);

        while self.offset < self.chars_len {
            let ch = self.chars[self.offset];

            if ch == '\n' {
                break;
            }

            let token = match ch {
                '*' | '`' => self.get_styled_text_token(),
                _ => self.parse_text(),
            };

            tokens.push(token);
        }

        tokens
    }

    fn get_styled_text_token(&mut self) -> TextToken {
        //This special character will be either * or ` characters
        let spec_ch = self.chars[self.offset];
        let mut spec_characters = Vec::with_capacity(3);
        let mut ch_count = 0;

        // Calculate how many special characters we have to identify the style
        while self.offset < self.chars_len && ch_count < 3 && self.chars[self.offset] == spec_ch {
            spec_characters.push(spec_ch);
            self.offset += 1;
            ch_count += 1;
        }

        if self.offset == self.chars_len {
            return TextToken::Text("".to_string());
        }

        let spec_ch_str: String = spec_characters.into_iter().collect();

        self.stack.push(spec_ch_str);

        let mut tokens = Vec::with_capacity(10);

        while self.offset < self.chars_len {
            let ch = self.chars[self.offset];

            if ch == spec_ch {
                if let Some(last) = self.stack.last()
                    && self.is_styled_text_token_closure()
                {
                    self.offset += last.len();
                    return self.compact_text_token(tokens);
                } else {
                    let token = self.get_styled_text_token();
                    tokens.push(token);
                }
            } else {
                match ch {
                    '*' | '`' => tokens.push(self.get_styled_text_token()),
                    _ => tokens.push(self.parse_text()),
                }
            }
        }

        if let Some(_) = self.stack.last() {
            self.compact_text_token(tokens)
        } else {
            return TextToken::Text("".to_string());
        }
    }

    fn parse_text(&mut self) -> TextToken {
        let mut text_chs = Vec::with_capacity(100);

        while self.offset < self.chars_len {
            let ch = self.chars[self.offset];

            if ch == '`' || ch == '*' || ch == '\n' {
                break;
            }

            text_chs.push(ch);
            self.offset += 1;
        }

        let text_token_str: String = text_chs.into_iter().collect();

        TextToken::Text(text_token_str)
    }

    fn compact_text_token(&mut self, tokens: Vec<TextToken>) -> TextToken {
        let spec_ch_str = self.stack.pop().unwrap();

        match spec_ch_str.as_str() {
            "*" => TextToken::Italic(tokens),
            "**" => TextToken::Bold(tokens),
            "***" => TextToken::BoldItalic(tokens),
            "`" | "``" | "```" => TextToken::Code(tokens),
            _ => panic!("Invalid Special Character \"{}\"", spec_ch_str),
        }
    }

    fn is_styled_text_token_closure(&self) -> bool {
        let special_ch = self.chars[self.offset];
        let stack_str: String = self.stack.clone().into_iter().rev().collect();

        let mut offset = self.offset;
        let mut chs = Vec::with_capacity(3);

        while offset < self.chars_len && self.chars[offset] == special_ch {
            chs.push(self.chars[offset]);

            offset += 1;
        }

        let read_chs: String = chs.into_iter().collect();

        self.stack.last().unwrap() == &read_chs || read_chs == stack_str
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
        let result = markdown_parser.parse();

        assert_eq!(
            result,
            vec![MarkdownToken::Header(
                HeaderLevel::One,
                vec![TextToken::Text("Hello World!".to_string())]
            )]
        );
    }

    #[test]
    fn parse_header_1_with_style_children() {
        let input = "# Hello *World*!";

        let mut markdown_parser = MarkdownParser::new(input);
        let result = markdown_parser.parse();

        assert_eq!(
            result,
            vec![MarkdownToken::Header(
                HeaderLevel::One,
                vec![
                    TextToken::Text("Hello ".to_string()),
                    TextToken::Italic(vec![TextToken::Text("World".to_string())]),
                    TextToken::Text("!".to_string())
                ]
            )]
        );
    }

    #[test]
    fn parse_header_1_with_style_children_2() {
        let input = "# Hello *World **123***!";

        let mut markdown_parser = MarkdownParser::new(input);
        let result = markdown_parser.parse();

        assert_eq!(
            result,
            vec![MarkdownToken::Header(
                HeaderLevel::One,
                vec![
                    TextToken::Text("Hello ".to_string()),
                    TextToken::Italic(vec![
                        TextToken::Text("World ".to_string()),
                        TextToken::Bold(vec![TextToken::Text("123".to_string())])
                    ]),
                    TextToken::Text("!".to_string())
                ]
            )]
        );
    }

    #[test]
    fn parse_header_1_with_style_children_3() {
        let input = "# Hello *World **123** `i am a code`*!";

        let mut markdown_parser = MarkdownParser::new(input);
        let result = markdown_parser.parse();

        assert_eq!(
            result,
            vec![MarkdownToken::Header(
                HeaderLevel::One,
                vec![
                    TextToken::Text("Hello ".to_string()),
                    TextToken::Italic(vec![
                        TextToken::Text("World ".to_string()),
                        TextToken::Bold(vec![TextToken::Text("123".to_string())]),
                        TextToken::Text(" ".to_string()),
                        TextToken::Code(vec![TextToken::Text("i am a code".to_string())])
                    ]),
                    TextToken::Text("!".to_string())
                ]
            )]
        );
    }

    #[test]
    fn parse_header_1_with_style_children_4() {
        let input = "# **Introduction to** ***Programming*** with `Rust`";

        let mut markdown_parser = MarkdownParser::new(input);
        let result = markdown_parser.parse();

        assert_eq!(
            result,
            vec![MarkdownToken::Header(
                HeaderLevel::One,
                vec![
                    TextToken::Bold(vec![TextToken::Text("Introduction to".to_string())]),
                    TextToken::Text(" ".to_string()),
                    TextToken::BoldItalic(vec![TextToken::Text("Programming".to_string())]),
                    TextToken::Text(" with ".to_string()),
                    TextToken::Code(vec![TextToken::Text("Rust".to_string())]),
                ]
            )]
        );
    }

    #[test]
    fn parse_header_1_with_style_children_5() {
        let input = "# **Introduction to** ***Programming*** with `Rust *Programming Language*`";

        let mut markdown_parser = MarkdownParser::new(input);
        let result = markdown_parser.parse();

        assert_eq!(
            result,
            vec![MarkdownToken::Header(
                HeaderLevel::One,
                vec![
                    TextToken::Bold(vec![TextToken::Text("Introduction to".to_string())]),
                    TextToken::Text(" ".to_string()),
                    TextToken::BoldItalic(vec![TextToken::Text("Programming".to_string())]),
                    TextToken::Text(" with ".to_string()),
                    TextToken::Code(vec![
                        TextToken::Text("Rust ".to_string()),
                        TextToken::Italic(vec![TextToken::Text(
                            "Programming Language".to_string()
                        )])
                    ]),
                ]
            )]
        );
    }

    #[test]
    fn parse_header_1_with_newline() {
        let input = "# Hello World!\n\n";

        let mut markdown_parser = MarkdownParser::new(input);
        let result = markdown_parser.parse();

        assert_eq!(
            result,
            vec![
                MarkdownToken::Header(
                    HeaderLevel::One,
                    vec![TextToken::Text("Hello World!".to_string())]
                ),
                MarkdownToken::NewLine,
                MarkdownToken::NewLine
            ]
        );
    }

    #[test]
    fn parse_header_2() {
        let input = "## **Hello World!**";

        let mut markdown_parser = MarkdownParser::new(input);
        let result = markdown_parser.parse();

        assert_eq!(
            result,
            vec![MarkdownToken::Header(
                HeaderLevel::Two,
                vec![TextToken::Bold(vec![TextToken::Text(
                    "Hello World!".to_string()
                )])]
            )]
        );
    }
}
