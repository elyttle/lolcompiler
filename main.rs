
fn main (){

}

/// Trait for a simple lolcompiler front-end.
/// Errors should cause immediate exit inside the implementation.
pub traits::Compiler for Compiler{
    /// Begin the compilation process (entry point).
    fn compile(&mut self, source: &str){
        self.lexer = LexicalAnalyzer::new(source);
        self.current_token = self.lexer.start();
        self.parse();
    }
    /// Get the next token from the lexical analyzer.
    fn next_token(&mut self) -> String{
        let tok = self.next();
        self.current_token = tok.clone();
        tok
    }
    /// Run the syntax analyzer starting from <lolcode>.
    fn parse(&mut self){
        let mut parser = SyntaxAnalyzer::new(self);
        parser.parse_lolcode();
        if !self.current_token.is_empty(){
            eprintln!("Syntax error: token '{}' found after #KBYE ",self.current_token);
            std::process::exit(1);
        }
    }
    /// Get the current token being processed.
    fn current_token(&self) -> String{self.current_token.clone()}
    /// Set the current token (typically used internally).
    fn set_current_token(&mut self, tok: String){self.current_token = tok;}
}

pub struct Compiler {
    pub current_token: String,
    lexer: LexicalAnalyzer,
    pub html:String,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            current_token: String::new(), lexer: LexicalAnalyzer::new(""), html: String::new()
        }
    }
}

/// OPTION 1 - Trait for a recursive descent Syntax Analyzer
/// over Vec<String>. Each function parses a nonterminal in
/// the grammar. On error: exit immediately.
pub trait SyntaxAnalyzer {
    fn parse_lolcode(&mut self);
    fn parse_head(&mut self);
    fn parse_title(&mut self);
    fn parse_comment(&mut self);
    fn parse_body(&mut self);
    fn parse_paragraph(&mut self);
    fn parse_inner_paragraph(&mut self);
    fn parse_inner_text(&mut self);
    fn parse_variable_define(&mut self);
    fn parse_variable_use(&mut self);
    fn parse_bold(&mut self);
    fn parse_list(&mut self);
    fn parse_list_items(&mut self);
    fn parse_inner_list(&mut self);
    fn parse_link(&mut self);
    fn parse_newline(&mut self);
    fn parse_text(&mut self);
}

pub struct LexicalAnalyzer{
    input: Vec<char>,
    position: usize,
    current_build: String,
    tokens: Vec<String>,
    pub keywords: Vec<String>,
}

pub traits::LexicalAnalyzer for LexicalAnalyzer{
    /// Return the next character from the input.
    /// If input is exhausted, should terminate the program.
    fn get_char(&mut self)->char{
        if self.position<self.input.len(){
            let c = self.input[self.position];
            self.position+=1;
            c
        }
        else{
            '\0'
        }
    }
    /// Add a character to the current potential token.
    fn add_char(&mut self, c:char){
        self.current_build.push(c);
    }
    /// Lookup a potential token to determine if it is valid.
    /// Returns true if a valid token/lexeme, false otherwise.
    fn lookup(&self, word: &str)-> bool{
        self.keywords.iter().any(|k|k==s)
    }
}

impl LexicalAnalyzer{
    pub fn new(input: &str) -> Self{
        let mut lexer = Self{
            input: input.chars().collect(),
            position: 0,
            current_build: String::new(),
            tokens: Vec::new(),
            keywords: vec![
                "#HAI".into(), "#KBYE".into(), "#OBTW".into(), "#TLDR".into(), "#MAEK HEAD".into(), "#MAEK PARAGRAF".into(),
                "#MAEK LIST".into(), "#GIMMEH TITLE".into(), "#GIMMEH BOLD".into(), "#GIMMEH ITEM".into(), "#GIMMEH LINX".into(),
                "#GIMMEH VAR".into(), "#IHAZ".into(), "#ITIZ".into(), "#MKAY".into(), "#OIC".into(), "#NEWLINE".into(),
            ],
        }
        lexer.tokenize();
        lexer
    }

    pub fn tokenize(&mut self){
        loop{
            let c = self.get_char();
            if c=='\0'{ //nothing left
                break;
            }

            if c.is_whitespace() {
                if !self.current_build.is_empty() {
                    self.tokens
                        .push(std::mem::take(&mut self.current_build));
                }
            } 
            else if c == '#'{
                if !self.current_build.is_empty() {
                    self.tokens
                        .push(std::mem::take(&mut self.current_build));
                }
                self.add_char(c);
            }
            else {
                self.add_char(c);
            }
        }

        if !self.current_build.is_empty() {
            self.tokens
                .push(std::mem::take(&mut self.current_build));
        }

        //for "#MAEK HEAD" "#GIMMEH TITLE" and the like
        let mut merged: Vec<String> = Vec::new();
        let mut i = 0;
        while i < self.tokens.len(){
            let word = self.tokens[i].clone();
            if (word == "#MAEK" || word == "#GIMMEH") && i+1 < self.tokens.len(){
                merged.push(format!("{} {}",word,self.tokens[i+1]));
                i+=2;
            }
            else{
                merged.push(word);
                i+=1;
            }
        }
        self.tokens=merged;
        self.tokens.reverse();
    }

    //checking for legal characters
    pub fn is_valid_word(&self, word: &str) -> bool{
        for c in word.chars(){
            let ok = (c>='A'&&c<='Z')||(c>='a'&&c<='z')||(c>='0'&&c<='9')||c==','||c=='.'||c=='"'||c==':'||c=='?'||c=='!'||c=='%'||c=='='||c=='/';
            if !ok{
                return false;
            }
        }
        true
    }

    pub fn start (&mut self)->String{
        let candidate_token = self.tokens.pop().unwrap_or_default();
        if candidate_token.is_empty(){
            eprintln!("Error occured: file is empty");
            std::process::exit(1);
        }
        self.count+=1;
        self.check(candidate_token)
    }

    pub fn next(& mut self)-> String{
        if self.tokens.is_empty(){
            return "".to_string();
        }
        let candidate_token = self.tokens.pop().unwrap_or_default();
        self.check(candidate_token)
    }

    fn check(&self, candidate_token:String)->String{
        //token legitimacy check
        if candidate_token.starts_with('#'){
            if self.lookup(&candidate_token){
                return candidate_token;
            }
            eprintln!("Lexical error occured, '{}'is not recognized token",candidate_token);
            std::process::exit(1);
        }

        //character legality check
        if self.is_valid_word(&candidate_token){
            candidate_token
        }
        else{
            eprintln!("Lexical error occured '{}' contains an illegal character",candidate_token);
            std::process::exit(1);
        }
    }
}
