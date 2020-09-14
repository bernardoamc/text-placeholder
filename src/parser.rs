enum State {
    Text,
    Placeholder,
}

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
            token = Token::Placeholder(&self.text[self.start.len()..placeholder_index]);
            let new_position = placeholder_index + self.end.len();
            self.text = &self.text[new_position..];
        } else {
            token = Token::Text(self.text);
            self.text = "";
        }

        token
    }
}
