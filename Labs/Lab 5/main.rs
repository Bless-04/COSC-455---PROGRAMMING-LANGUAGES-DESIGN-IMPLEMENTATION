pub struct Compiler {
    pub current_token: String,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            current_token: String::new(),
        }
    }
}

pub struct LexicalAnalyzer {
    tokens: Vec<String>,
    pub articles: Vec<String>,
    pub adjectives: Vec<String>,
    pub verbs: Vec<String>,
    pub nouns: Vec<String>,
}

impl LexicalAnalyzer {
    pub fn new(input: &str) -> Self {
        //puts a bunch of provided words into words so we can use themg3
        let mut tokens: Vec<String> = input
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        tokens.reverse();

        Self {
            tokens,
            articles: vec!["a".into(), "teh".into()],
            adjectives: vec!["fat".into(), "hungry".into(), "happy".into(), "mean".into()],
            verbs: vec!["lovez".into(), "hatez".into(), "ates".into()],
            nouns: vec!["dawg".into(), "kat".into(), "rat".into()],
        }
    }

    pub fn lookup(&mut self, word: &str) -> bool {
        self.articles.iter().any(|a| a == word)
            || self.adjectives.iter().any(|j| j == word)
            || self.nouns.iter().any(|n| n == word)
            || self.verbs.iter().any(|v| v == word)
    }

    pub fn start(&mut self) -> String {
        let candidate_token = self.tokens.pop().unwrap_or_default();

        if self.lookup(&candidate_token) {
            candidate_token
        } else if !candidate_token.is_empty() {
            eprintln!(
                "A lexical error was encountered. '{}' is not a recognized token.",
                candidate_token
            );
            std::process::exit(1);
        } else {
            eprintln!("A user error was encountered. The provided sentence is empty.");
            std::process::exit(1);
        }
    }

    pub fn next(&mut self) -> String {
        let candidate_token = self.tokens.pop().unwrap_or_default();

        if self.lookup(&candidate_token) {
            candidate_token
        } else if self.tokens.is_empty() {
            "".to_string()
        } else {
            eprintln!(
                "A lexical error was encountered. '{}' is not a recognized token.",
                candidate_token
            );
            std::process::exit(1);
        }
    }

    pub fn is_a_article(&self, word: &str) -> bool {
        self.articles.iter().any(|a| a == word)
    }

    pub fn is_a_noun(&self, word: &str) -> bool {
        self.nouns.iter().any(|n| n == word)
    }

    pub fn is_a_verb(&self, word: &str) -> bool {
        self.verbs.iter().any(|v| v == word)
    }

    pub fn is_a_adjective(&self, word: &str) -> bool {
        self.adjectives.iter().any(|v| v == word)
    }
}

pub struct SyntaxAnalyzer<'a> {
    lexer: &'a mut LexicalAnalyzer,
    compiler: &'a mut Compiler,
}

impl<'a> SyntaxAnalyzer<'a> {
    pub fn new(lexer: &'a mut LexicalAnalyzer, compiler: &'a mut Compiler) -> Self {
        Self { lexer, compiler }
    }

    pub fn next_token(&mut self) {
        let tok = self.lexer.next();
        self.compiler.current_token = tok;
    }

    pub fn lolspeak(&mut self) {
        self.noun_phrase();
        self.verb();
        self.noun_phrase();
    }

    /* After task 3 a noun phrase is an article followed by an adjective followed by a noun */
    pub fn noun_phrase(&mut self) {
        self.article();
        self.adjective();
        self.noun();
    }

    pub fn verb(&mut self) {
        if self.lexer.is_a_verb(&self.compiler.current_token) {
            self.next_token();
        } else {
            eprintln!(
                "A syntax error was encountered. {} was found when an verb was expected",
                self.compiler.current_token
            );
            std::process::exit(1);
        }
    }

    pub fn noun(&mut self) {
        if self.lexer.is_a_noun(&self.compiler.current_token) {
            self.next_token();
        } else {
            eprintln!(
                "A syntax error was encounted. {} was found when an noun was expected",
                &self.compiler.current_token
            );
            std::process::exit(1);
        }
    }

    pub fn article(&mut self) {
        // if current token is an article
        if self.lexer.is_a_article(&self.compiler.current_token) {
            self.next_token(); //asks lexer to get next token
        } else {
            eprintln!(
                "A Syntax error was encountered. {} was found when an article was expected",
                self.compiler.current_token
            );
            std::process::exit(1);
        }
    }

    pub fn adjective(&mut self) {
        if self.lexer.is_a_adjective(&self.compiler.current_token) {
            self.next_token();
        } else {
            eprintln!(
                "A Syntax error was encountered. {} was found when an adjective was expected",
                self.compiler.current_token
            );
            std::process::exit(1);
        }
    }
}

fn main() {
    let test1 = "a kat lovez teh dawg";

    let sentence = test1; //"a kat teh dog";

    //create instances of lexical analyzer
    let mut compiler = Compiler::new();
    let mut lexer = LexicalAnalyzer::new(sentence);

    compiler.current_token = lexer.start(); //where the work starts for lab 5.

    let mut parser = SyntaxAnalyzer::new(&mut lexer, &mut compiler);
    parser.lolspeak();

    if !lexer.tokens.is_empty() || !compiler.current_token.is_empty() {
        eprintln!("A syntax error was encountered. Additional tokens found after the sentence.");
        std::process::exit(1);
    }
    println!("{:?}", lexer.tokens);
    println!("The sentence '{}' follows the lolspeak grammar!", sentence);
}
