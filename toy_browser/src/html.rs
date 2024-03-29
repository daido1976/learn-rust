use crate::dom;
use std::collections::HashMap;

/// Parse an HTML document and return the root element.
pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser::new(0, source).parse_nodes();

    // If the document contains a root element, just return it. Otherwise, create one.
    if nodes.len() == 1 {
        nodes.swap_remove(0)
    } else {
        dom::elem("html".to_string(), HashMap::new(), nodes)
    }
}

struct Parser {
    position: usize,
    input: String,
}

impl Parser {
    /// Creates a new `Parser`.
    fn new(position: usize, input: String) -> Self {
        Parser { position, input }
    }

    /// Parse a sequence of sibling nodes.
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = vec![];
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        nodes
    }

    /// Parse a single node.
    fn parse_node(&mut self) -> dom::Node {
        match self.current_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    /// Parse a single element, including its open tag, contents, and closing tag.
    fn parse_element(&mut self) -> dom::Node {
        // Opening tag.
        assert_eq!(self.consume_char(), '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert_eq!(self.consume_char(), '>');

        // Contents.
        let children = self.parse_nodes();

        // Closing tag.
        assert_eq!(self.consume_char(), '<');
        assert_eq!(self.consume_char(), '/');
        assert_eq!(self.parse_tag_name(), tag_name);
        assert_eq!(self.consume_char(), '>');

        dom::elem(tag_name, attrs, children)
    }

    /// Parse a tag or attribute name.
    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9'))
    }

    /// Parse a list of name="value" pairs, separated by whitespace.
    fn parse_attributes(&mut self) -> dom::AttributeMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.current_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attribute();
            attributes.insert(name, value);
        }
        attributes
    }

    /// Parse a single name="value" pair.
    fn parse_attribute(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert_eq!(self.consume_char(), '=');
        let value = self.parse_attribute_value();
        (name, value)
    }

    /// Parse a quoted value.
    fn parse_attribute_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert_eq!(self.consume_char(), open_quote);
        value
    }

    /// Parse a text node.
    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c| c != '<'))
    }

    /// Consume and discard zero or more whitespace characters.
    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    /// Consume characters until `predicate` returns false.
    fn consume_while<F>(&mut self, predicate: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && predicate(self.current_char()) {
            result.push(self.consume_char());
        }
        result
    }

    /// Return the current character, and advance self.position to the next character.
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.position..].char_indices();
        let (_, current_char) = iter.next().expect("reach the eof");
        let (next_position, _) = iter.next().unwrap_or((1, ' '));
        self.position += next_position;
        current_char
    }

    // Read the current character without consuming it.
    fn current_char(&self) -> char {
        self.input.chars().nth(self.position).unwrap_or_default()
    }

    // Do the next characters start with the given string?
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.position..].starts_with(s)
    }

    // Return true if all input is consumed.
    fn eof(&self) -> bool {
        self.position >= self.input.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_element() {
        {
            let mut parser = Parser::new(
                0,
                "<p id=\"test\" class=\"sample\">hello world</p>".to_owned(),
            );
            let mut attributes = dom::AttributeMap::new();
            attributes.insert("id".to_string(), "test".to_string());
            attributes.insert("class".to_string(), "sample".to_string());
            let node = dom::elem(
                "p".to_owned(),
                attributes,
                vec![dom::text("hello world".to_owned())],
            );
            assert_eq!(parser.parse_element(), node);
        }
    }

    #[test]
    fn test_parse_attributes() {
        let mut parser = Parser::new(0, "test=\"foobar\" abc=\"def\">".to_owned());
        let mut attributes = dom::AttributeMap::new();
        attributes.insert("test".to_string(), "foobar".to_string());
        attributes.insert("abc".to_string(), "def".to_string());
        assert_eq!(parser.parse_attributes(), attributes);
    }

    #[test]
    fn test_parse_attribute() {
        let mut parser = Parser::new(0, "test=\"foobar\"".to_owned());
        assert_eq!(
            parser.parse_attribute(),
            ("test".to_string(), "foobar".to_string())
        );
    }

    #[test]
    fn test_consume_char() {
        let mut parser = Parser::new(3, "abcde".to_owned());
        let current_char = parser.consume_char();
        assert_eq!(current_char, 'd');
        assert_eq!(parser.position, 4)
    }

    #[test]
    fn test_current_char() {
        let parser = Parser::new(3, "abcde".to_owned());
        let current_char = parser.current_char();
        assert_eq!(current_char, 'd')
    }

    #[test]
    fn test_current_char_eof() {
        let parser = Parser::new(5, "abcde".to_owned());
        let current_char = parser.current_char();
        assert_eq!(current_char, '\0');
    }
}
