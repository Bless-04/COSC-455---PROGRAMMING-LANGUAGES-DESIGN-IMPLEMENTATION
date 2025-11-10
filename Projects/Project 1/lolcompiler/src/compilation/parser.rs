use std::collections::HashMap;

use crate::{
    SyntaxAnalyzer,
    compilation::{lexer::LolLexer, token::Token},
};

// Syntax Analyzer struct for lolcode ; middle step of compiling ; 2
pub struct LolParser<'a> {
    _lexer: LolLexer<'a>,
    _html: String,
    _variables: HashMap<String, String>, // vardef, varval
}

impl<'a> LolParser<'a> {
    pub fn new(mut token_source: LolLexer<'a>) -> Self {
        token_source.start(); // Tokenize the input
        Self {
            _lexer: token_source,
            _html: String::new(),
            _variables: HashMap::new(),
        }
    }

    /// Get the generated HTML
    pub fn get_html(&self) -> &str {
        &self._html
    }

    /// Helper function to expect a specific token
    fn expect_token(&mut self, expected: &Token) -> Result<(), String> {
        match self._lexer.next_token() {
            Some(token) => {
                if token == expected {
                    Ok(())
                } else {
                    Err(format!("Expected {:?}, found {:?}", expected, token))
                }
            }
            None => Err(format!("Expected {:?}, but reached end of input", expected)),
        }
    }

    /// Helper function to peek at the next token
    fn peek_token(&self) -> Option<&Token<'a>> {
        self._lexer.peek_token()
    }
}

impl<'a> SyntaxAnalyzer for LolParser<'a> {
    // #HAI (optional comments) <head> <body> #KTHXBYE
    fn parse_lolcode(&mut self) -> Result<(), String> {
        // Expect #HAI
        self.expect_token(&Token::HAI)?;
        self._html.push_str("<html>\n");

        // Optional comments
        while let Some(Token::OBTW) = self.peek_token() {
            self.parse_comment()?;
        }

        // Parse head
        self.parse_head()?;

        // Parse body
        self.parse_body()?;

        // Expect #KTHXBYE
        self.expect_token(&Token::KTHXBYE)?;
        self._html.push_str("</html>\n");

        // Should be at end
        match self.peek_token() {
            Some(Token::End) => Ok(()),
            Some(token) => Err(format!("Unexpected token after KTHXBYE: {:?}", token)),
            None => Ok(()),
        }
    }

    // #MAEK HEAD ... #OIC
    fn parse_head(&mut self) -> Result<(), String> {
        self.expect_token(&Token::MAEK)?;
        self.expect_token(&Token::HEAD)?;
        self._html.push_str("\t<head>\n");

        // Parse title
        self.parse_title()?;

        self.expect_token(&Token::OIC)?;
        self._html.push_str("\t</head>\n");
        Ok(())
    }

    // #GIMMEH TITLE text #MKAY
    fn parse_title(&mut self) -> Result<(), String> {
        self.expect_token(&Token::GIMMEH)?;
        self.expect_token(&Token::TITLE)?;
        self._html.push_str("\t\t<title>");

        let mut found_text = false;

        while let Some(Token::Text(_text)) = self.peek_token() {
            let token = self._lexer.next_token().unwrap();
            if let Token::Text(t) = token {
                self._html.push_str(" ");
                self._html.push_str(t);
            }
            found_text = true;
        }

        if !found_text {
            return Err("Expected text after TITLE".to_string());
        }

        self.expect_token(&Token::MKAY)?;
        self._html.push_str(" </title>\n");
        Ok(())
    }

    // #OBTW ... #TLDR
    fn parse_comment(&mut self) -> Result<(), String> {
        self.expect_token(&Token::OBTW)?;
        self._html.push_str("\t<!--");

        // Consume all tokens until #TLDR
        while let Some(token) = self._lexer.next_token() {
            if let Token::TLDR = token {
                self._html.push_str(" -->\n");
                return Ok(());
            }
            // Add text from comment tokens to HTML
            match token {
                Token::Text(t) => {
                    self._html.push_str(" ");
                    self._html.push_str(t);
                }
                _ => {}
            }
        }

        Err("Comment not closed with #TLDR".to_string())
    }

    // Body contains paragraphs, variables, text, lists, audio, video, newlines
    fn parse_body(&mut self) -> Result<(), String> {
        loop {
            match self.peek_token() {
                // look at next token without to for the spaced words
                Some(Token::MAEK) => {
                    // Could be paragraph or other things after MAEK ; todo: check later
                    self.parse_paragraph()?;
                }
                Some(Token::I) => {
                    self.parse_variable_define()?;
                }
                Some(Token::LEMME) => {
                    self.parse_variable_use()?;
                }
                Some(Token::GIMMEH) => {
                    // Body can contain GIMMEH directly (e.g., embedded in text)
                    self._lexer.next_token(); // consume GIMMEH
                    match self.peek_token() {
                        Some(Token::BOLD) => {
                            self._lexer.next_token();
                            self.parse_bold()?;
                        }
                        Some(Token::ITALICS) => {
                            self._lexer.next_token();
                            self.parse_italics()?;
                        }
                        Some(Token::LIST) => {
                            self._lexer.next_token();
                            self.parse_list()?;
                        }
                        _ => {
                            return Err(
                                "Expected BOLD, ITALICS, or LIST after GIMMEH in body".to_string()
                            );
                        }
                    }
                }
                Some(Token::SOUNDZ) => {
                    self._lexer.next_token();
                    self.parse_audio()?;
                }
                Some(Token::VIDZ) => {
                    self._lexer.next_token();
                    self.parse_video()?;
                }
                Some(Token::NEWLINE) => {
                    self.parse_newline()?;
                }
                Some(Token::Text(_)) => {
                    // Parse text which can contain embedded formatting
                    self.parse_inner_text()?;
                }
                Some(Token::KTHXBYE) | Some(Token::End) => {
                    break;
                }
                Some(token) => {
                    return Err(format!("Unexpected token in body: {:?}", token));
                }
                None => break,
            }
        }
        Ok(())
    }

    // #MAEK PARAGRAF ... #OIC
    fn parse_paragraph(&mut self) -> Result<(), String> {
        self.expect_token(&Token::MAEK)?;
        self.expect_token(&Token::PARAGRAF)?;

        match self.peek_token() {
            Some(Token::MKAY) => {
                return Err(
                    "Syntax error: PARAGRAF cannot be closed with MKAY, use OIC".to_string()
                );
            }
            Some(Token::OIC) => {
                // Empty paragraph
                self.expect_token(&Token::OIC)?;
                self._html.push_str("\t<p></p>\n");
                return Ok(());
            }
            _ => {
                // html parse for paragraph
                self._html.push_str("\t<p>");
                self.parse_inner_paragraph()?;
                self.expect_token(&Token::OIC)?;
                self._html.push_str("</p>\n");
            }
        }
        Ok(())
    }

    // Inner paragraph: can contain inner text, variables, bold, italics, lists
    fn parse_inner_paragraph(&mut self) -> Result<(), String> {
        loop {
            match self.peek_token() {
                Some(Token::GIMMEH) => {
                    self._lexer.next_token(); // consume GIMMEH
                    match self.peek_token() {
                        Some(Token::BOLD) => {
                            self._lexer.next_token(); // consume BOLD
                            self.parse_bold()?;
                        }
                        Some(Token::ITALICS) => {
                            self._lexer.next_token(); // consume ITALICS
                            self.parse_italics()?;
                        }
                        Some(Token::LIST) => {
                            self._lexer.next_token(); // consume LIST
                            self.parse_list()?;
                        }
                        Some(Token::NEWLINE) => {
                            self._lexer.next_token();
                            self.parse_newline()?;
                        }
                        Some(Token::SOUNDZ) => {
                            self._lexer.next_token();
                            self.parse_audio()?;
                        }
                        Some(Token::VIDZ) => {
                            self._lexer.next_token();
                            self.parse_video()?;
                        }
                        _ => {
                            return Err("Expected BOLD, ITALICS, LIST, NEWLINE, SOUNDZ, or VIDZ after GIMMEH".to_string());
                        }
                    }
                }
                Some(Token::I) => {
                    self.parse_variable_define()?;
                }
                Some(Token::LEMME) => {
                    self.parse_variable_use()?;
                }
                Some(Token::Text(_)) => {
                    self.parse_inner_text()?;
                }
                Some(Token::NEWLINE) => {
                    self.parse_newline()?;
                }
                Some(Token::OIC) => {
                    break;
                }
                Some(Token::MKAY) => {
                    // MKAY can appear in inner paragraph if it's closing a nested construct
                    // We'll let the calling function handle it
                    break;
                }
                Some(token) => {
                    return Err(format!("Unexpected token in inner paragraph: {:?}", token));
                }
                None => break,
            }
        }
        Ok(())
    }

    // Inner text: text tokens, bold, italics, variables, newlines
    fn parse_inner_text(&mut self) -> Result<(), String> {
        loop {
            match self.peek_token() {
                Some(Token::Text(_)) => {
                    self.parse_text()?;
                }
                Some(Token::GIMMEH) => {
                    self._lexer.next_token(); // consume GIMMEH
                    match self.peek_token() {
                        Some(Token::BOLD) => {
                            self._lexer.next_token();
                            self.parse_bold()?;
                        }
                        Some(Token::ITALICS) => {
                            self._lexer.next_token();
                            self.parse_italics()?;
                        }
                        Some(Token::NEWLINE) => {
                            self._lexer.next_token();
                            self.parse_newline()?;
                        }
                        Some(Token::SOUNDZ) => {
                            self._lexer.next_token();
                            self.parse_audio()?;
                        }
                        Some(Token::VIDZ) => {
                            self._lexer.next_token();
                            self.parse_video()?;
                        }
                        _ => break, // Not a text formatting token
                    }
                }
                Some(Token::LEMME) => {
                    self.parse_variable_use()?;
                }
                Some(Token::NEWLINE) => {
                    self.parse_newline()?;
                }
                _ => break, // End of inner text
            }
        }
        Ok(())
    }

    // #I HAZ VarDef #IT IZ VarVal #MKAY
    fn parse_variable_define(&mut self) -> Result<(), String> {
        self.expect_token(&Token::I)?;
        self.expect_token(&Token::HAZ)?;

        // After HAZ, the next token should be VarDef (variable name)
        // still works if its text for some reason??
        let var_name = match self._lexer.next_token() {
            Some(Token::VarDef(name)) => name.to_string(),
            Some(Token::Text(name)) => name.to_string(),
            Some(token) => {
                return Err(format!(
                    "Expected variable name after HAZ, found {:?}",
                    token
                ));
            }
            None => return Err("Expected variable name after HAZ, but reached end".to_string()),
        };

        self.expect_token(&Token::IT)?;
        self.expect_token(&Token::IZ)?;

        // After IZ, the next token should be VarVal (variable value)
        // But if lexer tokenized it as Text, we'll accept that too
        let var_value = match self._lexer.next_token() {
            Some(Token::VarVal(value)) => value.to_string(),
            Some(Token::Text(value)) => value.to_string(),
            Some(token) => {
                return Err(format!(
                    "Expected variable value after IZ, found {:?}",
                    token
                ));
            }
            None => return Err("Expected variable value after IZ, but reached end".to_string()),
        };

        // Store the variable
        self._variables.insert(var_name, var_value);

        self.expect_token(&Token::MKAY)?;
        Ok(())
    }

    // #LEMME SEE VarDef #MKAY
    fn parse_variable_use(&mut self) -> Result<(), String> {
        self.expect_token(&Token::LEMME)?;
        self.expect_token(&Token::SEE)?;

        // After SEE, the next token should be VarDef (variable name)
        // But if lexer tokenized it as Text, we'll accept that too
        let var_name = match self._lexer.next_token() {
            Some(Token::VarDef(name)) => name.to_string(),
            Some(Token::Text(name)) => name.to_string(),
            Some(token) => {
                return Err(format!(
                    "Expected variable name after SEE, found {:?}",
                    token
                ));
            }
            None => return Err("Expected variable name after SEE, but reached end".to_string()),
        };

        // Look up and output the variable value
        if let Some(value) = self._variables.get(&var_name) {
            self._html.push_str(value);
        } else {
            return Err(format!("Undefined variable: {}", var_name));
        }

        self.expect_token(&Token::MKAY)?;
        Ok(())
    }

    // #GIMMEH BOLD text #MKAY
    fn parse_bold(&mut self) -> Result<(), String> {
        // BOLD token already consumed
        self._html.push_str("<b>");

        // Expect text
        let mut found_text = false;
        while let Some(Token::Text(_text)) = self.peek_token() {
            let token = self._lexer.next_token().unwrap();
            if let Token::Text(t) = token {
                self._html.push_str(" ");
                self._html.push_str(t);
            }
            found_text = true;
        }

        if !found_text {
            return Err("Expected text after BOLD".to_string());
        }

        self.expect_token(&Token::MKAY)?;
        self._html.push_str("</b>");
        Ok(())
    }

    // #GIMMEH ITALICS text #MKAY
    fn parse_italics(&mut self) -> Result<(), String> {
        // ITALICS token already consumed
        self._html.push_str("<i>");

        // Expect text
        let mut found_text = false;
        while let Some(Token::Text(_text)) = self.peek_token() {
            let token = self._lexer.next_token().unwrap();
            if let Token::Text(t) = token {
                self._html.push_str(" ");
                self._html.push_str(t);
            }
            found_text = true;
        }

        if !found_text {
            return Err("Expected text after ITALICS".to_string());
        }

        self.expect_token(&Token::MKAY)?;
        self._html.push_str("</i>");
        Ok(())
    }

    // #GIMMEH LIST ... #MKAY
    fn parse_list(&mut self) -> Result<(), String> {
        // LIST token already consumed
        self._html.push_str("\t\t<ul>\n");
        self.parse_list_items()?;
        self.expect_token(&Token::MKAY)?;
        self._html.push_str("\t\t</ul>");
        Ok(())
    }

    // LIST ITEM ... (repeated)
    fn parse_list_items(&mut self) -> Result<(), String> {
        while let Some(Token::GIMMEH) = self.peek_token() {
            self._lexer.next_token(); // consume GIMMEH
            match self.peek_token() {
                Some(Token::ITEM) => {
                    self._lexer.next_token(); // consume ITEM
                    self._html.push_str("\t\t\t<li>");
                    self.parse_inner_list()?;
                    self._html.push_str("</li>\n");
                }
                _ => break, // Not an ITEM, end of list
            }
        }
        Ok(())
    }

    // Inner list: text, bold, italics, variables
    fn parse_inner_list(&mut self) -> Result<(), String> {
        loop {
            match self.peek_token() {
                Some(Token::Text(_)) => {
                    self.parse_text()?;
                }
                Some(Token::GIMMEH) => {
                    self._lexer.next_token(); // consume GIMMEH
                    match self.peek_token() {
                        Some(Token::BOLD) => {
                            self._lexer.next_token();
                            self.parse_bold()?;
                        }
                        Some(Token::ITALICS) => {
                            self._lexer.next_token();
                            self.parse_italics()?;
                        }
                        _ => break,
                    }
                }
                Some(Token::LEMME) => {
                    self.parse_variable_use()?;
                }
                Some(Token::ITEM) | Some(Token::MKAY) => {
                    break; // End of this item
                }
                _ => break,
            }
        }
        Ok(())
    }

    // SOUNDZ Address
    fn parse_audio(&mut self) -> Result<(), String> {
        let address = match self._lexer.next_token() {
            Some(Token::Address(addr)) => addr,
            Some(token) => return Err(format!("Expected Address after SOUNDZ, found {:?}", token)),
            None => return Err("Expected Address after SOUNDZ, but reached end".to_string()),
        };

        self._html
            .push_str("<audio controls>\n\t\t\t <source src=\"");
        self._html.push_str(address);
        self._html.push_str("\">\n</audio>");
        Ok(())
    }

    // VIDZ Address
    fn parse_video(&mut self) -> Result<(), String> {
        let address = match self._lexer.next_token() {
            Some(Token::Address(addr)) => addr,
            Some(token) => return Err(format!("Expected Address after VIDZ, found {:?}", token)),
            None => return Err("Expected Address after VIDZ, but reached end".to_string()),
        };

        self._html.push_str("<iframe src=\"");
        self._html.push_str(address);
        self._html.push_str("\"/>\n");
        Ok(())
    }

    // NEWLINE
    fn parse_newline(&mut self) -> Result<(), String> {
        // NEWLINE token already consumed if called from parse_inner_text
        // Otherwise consume it here
        if let Some(Token::NEWLINE) = self.peek_token() {
            self._lexer.next_token();
        }
        self._html.push_str(" <br>\n\t\t");
        Ok(())
    }

    // Text(...)
    fn parse_text(&mut self) -> Result<(), String> {
        match self._lexer.next_token() {
            Some(Token::Text(text)) => {
                self._html.push_str(" ");
                self._html.push_str(text);
                Ok(())
            }
            Some(token) => Err(format!("Expected Text token, found {:?}", token)),
            None => Err("Expected Text token, but reached end".to_string()),
        }
    }
}
