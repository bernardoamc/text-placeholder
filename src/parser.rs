enum State {
    Text,
    Placeholder,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Token<'t> {
    Text(&'t str),
    Placeholder(&'t str),
}

pub struct Parser<'t> {
    text: &'t str,
    state: State,
    start: &'t str,
    end: &'t str,
}

impl<'t> Parser<'t> {
    pub fn new(text: &'t str, start: &'t str, end: &'t str) -> Self {
        Self {
            text,
            start,
            end,
            state: State::Text,
        }
    }

    pub fn parse(&mut self) -> Vec<Token<'t>> {
        let mut tokens: Vec<Token> = Vec::new();

        while self.text.len() > 0 {
            match self.state {
                State::Text => tokens.push(self.parse_text()),
                State::Placeholder => tokens.push(self.parse_placeholder()),
            }
        }

        tokens
    }

    fn parse_text(&mut self) -> Token<'t> {
        let token: Token;

        if let Some(placeholder_index) = self.text.find(self.start) {
            token = Token::Text(&self.text[..placeholder_index]);
            self.text = &self.text[placeholder_index..];
            self.state = State::Placeholder;
        } else {
            token = Token::Text(self.text);
            self.text = "";
        }

        token
    }

    fn parse_placeholder(&mut self) -> Token<'t> {
        let token: Token;
        self.state = State::Text;

        if let Some(placeholder_index) = self.text.find(self.end) {
            token = Token::Placeholder(
                &self.text[self.start.len()..placeholder_index]
                    .trim_start_matches(" ")
                    .trim_end_matches(" "),
            );
            let new_position = placeholder_index + self.end.len();
            self.text = &self.text[new_position..];
        } else {
            token = Token::Text(self.text);
            self.text = "";
        }

        token
    }
}

#[cfg(test)]
mod tests {
    use super::{Parser, Token};

    #[test]
    fn test_no_boundaries_present() {
        let tokens = Parser::new("hello world", "[", "]").parse();
        assert_eq!(tokens, vec![Token::Text("hello world")]);
    }

    #[test]
    fn test_boundary_begging_of_line() {
        let tokens = Parser::new("[placeholder] text", "[", "]").parse();
        assert_eq!(
            tokens,
            vec![
                Token::Text(""),
                Token::Placeholder("placeholder"),
                Token::Text(" text")
            ]
        );
    }

    #[test]
    fn test_boundary_middle_of_line() {
        let tokens = Parser::new("text [placeholder] text", "[", "]").parse();
        assert_eq!(
            tokens,
            vec![
                Token::Text("text "),
                Token::Placeholder("placeholder"),
                Token::Text(" text")
            ]
        );
    }

    #[test]
    fn test_boundary_end_of_line() {
        let tokens = Parser::new("text [placeholder]", "[", "]").parse();
        assert_eq!(
            tokens,
            vec![Token::Text("text "), Token::Placeholder("placeholder")]
        );
    }

    #[test]
    fn test_boundary_multiple_boundaries() {
        let tokens = Parser::new(
            "[placeholder] text [placeholder] test [placeholder]",
            "[",
            "]",
        )
        .parse();
        assert_eq!(
            tokens,
            vec![
                Token::Text(""),
                Token::Placeholder("placeholder"),
                Token::Text(" text "),
                Token::Placeholder("placeholder"),
                Token::Text(" test "),
                Token::Placeholder("placeholder")
            ]
        );
    }

    #[test]
    fn test_missing_boundary_start() {
        let tokens = Parser::new("text placeholder]", "[", "]").parse();
        assert_eq!(tokens, vec![Token::Text("text placeholder]")]);
    }

    #[test]
    fn test_missing_boundary_end() {
        let tokens = Parser::new("text [placeholder", "[", "]").parse();
        assert_eq!(
            tokens,
            vec![Token::Text("text "), Token::Text("[placeholder")]
        );
    }

    #[test]
    fn test_boundary_and_missing_boundaries() {
        let tokens = Parser::new("text [placeholder] [placeholder", "[", "]").parse();
        assert_eq!(
            tokens,
            vec![
                Token::Text("text "),
                Token::Placeholder("placeholder"),
                Token::Text(" "),
                Token::Text("[placeholder")
            ]
        );
    }

    #[test]
    fn test_multiple_chars_boundary_start_of_line() {
        let tokens = Parser::new("{{placeholder}} text", "{{", "}}").parse();
        assert_eq!(
            tokens,
            vec![
                Token::Text(""),
                Token::Placeholder("placeholder"),
                Token::Text(" text")
            ]
        );
    }

    #[test]
    fn test_multiple_chars_boundary_middle_of_line() {
        let tokens = Parser::new("text {{placeholder}} text", "{{", "}}").parse();
        assert_eq!(
            tokens,
            vec![
                Token::Text("text "),
                Token::Placeholder("placeholder"),
                Token::Text(" text")
            ]
        );
    }

    #[test]
    fn test_multiple_chars_boundary_end_of_line() {
        let tokens = Parser::new("text {{placeholder}}", "{{", "}}").parse();
        assert_eq!(
            tokens,
            vec![Token::Text("text "), Token::Placeholder("placeholder")]
        );
    }

    #[test]
    fn test_multiple_chars_boundary_multiple_boundaries() {
        let tokens = Parser::new(
            "{{placeholder}} text {{placeholder}} test {{placeholder}}",
            "{{",
            "}}",
        )
        .parse();
        assert_eq!(
            tokens,
            vec![
                Token::Text(""),
                Token::Placeholder("placeholder"),
                Token::Text(" text "),
                Token::Placeholder("placeholder"),
                Token::Text(" test "),
                Token::Placeholder("placeholder")
            ]
        );
    }

    #[test]
    fn test_multiple_chars_missing_boundary_start() {
        let tokens = Parser::new("text placeholder}}", "{{", "}}").parse();
        assert_eq!(tokens, vec![Token::Text("text placeholder}}")]);
    }

    #[test]
    fn test_multiple_chars_missing_boundary_end() {
        let tokens = Parser::new("text {{placeholder", "{{", "}}").parse();
        assert_eq!(
            tokens,
            vec![Token::Text("text "), Token::Text("{{placeholder")]
        );
    }

    #[test]
    fn test_multiple_chars_boundary_and_missing_boundaries() {
        let tokens = Parser::new("text {{placeholder}} {{placeholder", "{{", "}}").parse();
        assert_eq!(
            tokens,
            vec![
                Token::Text("text "),
                Token::Placeholder("placeholder"),
                Token::Text(" "),
                Token::Text("{{placeholder")
            ]
        );
    }

    #[test]
    fn test_trim_placeholdler_prefix_space() {
        let tokens = Parser::new("text [ placeholder]", "[", "]").parse();
        assert_eq!(
            tokens,
            vec![Token::Text("text "), Token::Placeholder("placeholder")]
        );
    }

    #[test]
    fn test_trim_placeholdler_suffix_space() {
        let tokens = Parser::new("text [placeholder ]", "[", "]").parse();
        assert_eq!(
            tokens,
            vec![Token::Text("text "), Token::Placeholder("placeholder")]
        );
    }

    #[test]
    fn test_trim_placeholdler_presuffix_space() {
        let tokens = Parser::new("text [ placeholder ]", "[", "]").parse();
        assert_eq!(
            tokens,
            vec![Token::Text("text "), Token::Placeholder("placeholder")]
        );
    }
}
