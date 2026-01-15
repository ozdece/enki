#[derive(Debug, PartialEq, Eq)]
enum ElementType {
    Italic,
    Bold,
    Text(String),
}

#[derive(Debug, PartialEq, Eq)]
struct Element {
    element_type: ElementType,
    children: Vec<Element>,
}

impl Element {
    
    pub fn element_type(&self) -> &ElementType {
        &self.element_type
    }

    pub fn append_child(&mut self, element: Element) {
        self.children.push(element);
    }

}

#[derive(Debug, PartialEq, Eq)]
enum Markdown {
    Paragraph(Element),
}

struct MarkdownParser {
    chars_len: usize,
    chars: Vec<char>,
    offset: usize,
    stack: Vec<Element>,
}

impl MarkdownParser {
    pub fn new(input: &str) -> Self {
        let chars_len = input.len();
        let chars: Vec<char> = input.chars().into_iter().collect();

        Self {
            chars_len,
            chars,
            offset: 0,
            stack: Vec::with_capacity(10),
        }
    }

    pub fn parse(&mut self) -> Vec<Markdown> {
        let element = self.parse_element();

        vec![Markdown::Paragraph(element)]
    }

    fn parse_element(&mut self) -> Element {
        if self.offset == self.chars_len {
            return self.get_empty_string_element();
        }

        let ch = self.chars[self.offset];

        match ch {
            '*' => self.parse_italic_or_bold(),
            _ => self.parse_text()
        }
    }

    fn parse_italic_or_bold(&mut self) -> Element {
        let styled_element_type = self.get_styled_element_type();

        match styled_element_type {
            Some(element_type) => {
                if let Some(element) = self.stack.last() && element.element_type() == &element_type {
                    return self.stack.pop().unwrap(); 
                }

                let new_element = Element {
                    element_type: element_type,
                    children: Vec::with_capacity(10)
                };

                self.stack.push(new_element);

                return self.parse_italic_or_bold();
            },
            None => {
                let mut chars = Vec::with_capacity(30);

                while self.offset < self.chars_len && self.chars[self.offset] != '*' && self.chars[self.offset] != '\n' {
                    chars.push(self.chars[self.offset]);
                    self.offset += 1;
                }

                let string = chars.into_iter().collect();
                let element_type = ElementType::Text(string);

                let element = Element {
                    element_type: element_type,
                    children: vec![]
                };

                if let Some(stack_elem) = self.stack.last_mut() {
                    stack_elem.children.push(element);
                    return self.parse_italic_or_bold();
                } else {
                    self.stack.push(element);
                    return self.parse_italic_or_bold();
                }
            },
        }
    }

    fn get_styled_element_type(&mut self) -> Option<ElementType> {
        let ch = self.chars[self.offset];

        match ch {
            '*' => {
                if self.offset + 1 < self.chars_len && self.chars[self.offset + 1] == '*' {
                    self.offset += 2;
                    Some(ElementType::Bold)
                } else {
                    self.offset += 1;
                    Some(ElementType::Italic)
                }
            }
            _ => None
        }
    }

    fn parse_text(&mut self) -> Element {
        let mut chars = Vec::with_capacity(30);

        //TODO: Check if character is a special character like *, **,_ etc.
        while self.offset < self.chars_len && self.chars[self.offset] != '\n' {
            chars.push(self.chars[self.offset]);
            self.offset += 1;
        }

        let string = chars.into_iter().collect();
        let element_type = ElementType::Text(string);

        Element {
            element_type: element_type,
            children: vec![],
        }
    }

    #[inline]
    fn get_empty_string_element(&self) -> Element {
        Element {
            element_type: ElementType::Text(String::from("")),
            children: vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_text() {
        let text = "Hello World!";
        let mut markdown_parser = MarkdownParser::new(text);

        let result = markdown_parser.parse();

        assert_eq!(
            result,
            vec![Markdown::Paragraph(Element {
                element_type: ElementType::Text(String::from("Hello World!")),
                children: vec![]
            })]
        )
    }

    #[test]
    fn parse_italic_text() {
        let text = "*Hello World!*";
        let mut markdown_parser = MarkdownParser::new(text);

        let result = markdown_parser.parse();

        assert_eq!(
            result,
            vec![Markdown::Paragraph(Element {
                element_type: ElementType::Italic,
                children: vec![Element {
                    element_type: ElementType::Text(String::from("Hello World!")),
                    children: vec![]
                }]
            })]
        )
    }

    #[test]
    fn parse_bold_text() {
        let text = "**Hello World!**";
        let mut markdown_parser = MarkdownParser::new(text);

        let result = markdown_parser.parse();

        assert_eq!(
            result,
            vec![Markdown::Paragraph(Element {
                element_type: ElementType::Bold,
                children: vec![Element {
                    element_type: ElementType::Text(String::from("Hello World!")),
                    children: vec![]
                }]
            })]
        )
    }

    #[test]
    fn parse_bold_italic_text() {
        let text = "***Hello World!***";
        let mut markdown_parser = MarkdownParser::new(text);

        let result = markdown_parser.parse();

        assert_eq!(
            result,
            vec![Markdown::Paragraph(Element {
                element_type: ElementType::Bold,
                children: vec![Element {
                    element_type: ElementType::Text(String::from("Hello World!")),
                    children: vec![]
                }]
            })]
        )
    }
}
