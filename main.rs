//Evan Lyttle
fn main (){

}

/// Trait for a simple lolcompiler front-end.
/// Errors should cause immediate exit inside the implementation.
pub trait Compiler {
    /// Begin the compilation process (entry point).
    fn compile(&mut self, source: &str);
    /// Get the next token from the lexical analyzer.
    fn next_token(&mut self) -> String;
    /// Run the syntax analyzer starting from <lolcode>.
    fn parse(&mut self);
    /// Get the current token being processed.
    fn current_token(&self) -> String;
    /// Set the current token (typically used internally).
    fn set_current_token(&mut self, tok: String);
}

/// Trait for a simple lexical analyzer.
/// Implements a character-by-character analysis
/// from a state machine design.
pub trait LexicalAnalyzer {
    /// Return the next character from the input.
    /// If input is exhausted, should terminate the program.
    fn get_char(&mut self) -> char;
    /// Add a character to the current potential token.
    fn add_char(&mut self, c: char);
    /// Lookup a potential token to determine if it is valid.
    /// Returns true if a valid token/lexeme, false otherwise.
    fn lookup(&self, s: &str) -> bool;
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

pub struct LexicalAnalyzer{
    input: Vec<char>,
    position: usize,
    current_build: String,
    tokens: Vec<String>,
    pub keywords: Vec<String>,
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

    pub fn get_char(&mut self)->char{
        if self.position<self.input.len(){
            let c = self.input[self.position];
            self.position++;
            c
        }
        else{
            '\0'
        }
    }

    pub fn add_char(&mut self, c:char){
        self.current_build.push(c);
    }

    pub fn tokenize(&mut self){
        loop{
            let c = self.get_char();
            if c=='\0'{ //nothing left
                break;
        }
        

    }
    
}