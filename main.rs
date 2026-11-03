use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

mod traits {
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
}

use traits::Compiler as _;
use traits::LexicalAnalyzer as _;
use traits::SyntaxAnalyzer as _;


/// Trait for a simple lolcompiler front-end.
/// Errors should cause immediate exit inside the implementation.
impl traits::Compiler for Compiler{
    /// Begin the compilation process (entry point).
    fn compile(&mut self, source: &str){
        self.lexer = LexicalAnalyzer::new(source);
        self.current_token = self.lexer.start();
        self.parse();
    }
    /// Get the next token from the lexical analyzer.
    fn next_token(&mut self) -> String{
        let tok = self.lexer.next();
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
            current_token: String::new(), 
            lexer: LexicalAnalyzer::new(""), 
            html: String::new(),
            count: 0,
        }
    }
}

pub struct LexicalAnalyzer{
    input: Vec<char>,
    position: usize,
    current_build: String,
    tokens: Vec<String>,
    pub keywords: Vec<String>,
    pub count: usize,
}

impl traits::LexicalAnalyzer for LexicalAnalyzer{
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
        self.keywords.iter().any(|k|k==word)
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
        };
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

        //makes two word steps into a single token
        let mut merged: Vec<String> = Vec::new();
        let mut i = 0;
        while i < self.tokens.len(){
            let word = self.tokens[i].clone();
            if (word.eq_ignore_ascii_case("#MAEK") || word.eq_ignore_ascii_case("#GIMMEH")) && i+1 < self.tokens.len(){
                merged.push(format!("{} {}", word, self.tokens[i+1]).to_uppercase());
                i+=2;
            }
            else if word.starts_with('#'){
                merged.push(word.to_uppercase()); // single-word annotations like #hai
                i+=1;
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
            let ok = (c>='A'&&c<='Z')||(c>='a'&&c<='z')||(c>='0'&&c<='9')||c==','||c=='.'||c=='"'||c==':'||c=='?'||c=='!'||c=='%'||c=='='||c=='/'||c=='-'||c=='_';
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
        self.count += 1;
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

pub struct SyntaxAnalyzer<'a>{
    compiler: &'a mut Compiler,
    scopes: Vec<Vec<(String, String)>>,
    last_text: String,
}

impl<'a>SyntaxAnalyzer<'a>{
    pub fn new(compiler:&'a mut Compiler) -> Self{
        Self{
            count: 0,
            compiler,
            scopes: vec![Vec::new()],//global scope
            last_text:String::new(),

        }
    }
    fn cur(&self)->String{
        self.compiler.current_token.clone()
    }
    fn advance(&mut self){
        self.compiler.next_token();
    }
    fn emit(&mut self, s:&str){
        self.compiler.html.push_str(s)
    }
    fn is_text(tok: &str) -> bool{
        !tok.is_empty()&&!tok.starts_with('#')
    }
    fn syntax_error(&self,expected: &str) -> !{
        let found = self.cur();
        let shown = if found.is_empty(){
            "end of file".to_string()
        }else{
            format!("'{}'",found)
        };
        eprintln!("Syntax error at token #{}: expected {} but found {}", self.compiler.lexer.count, expected, shown);
        std::process::exit(1);
    }

    //checks current token against expected token
    fn expect(&mut self, tok:&str){
        if self.cur()==tok{
            self.advance();
        }
        else{
            self.syntax_error(&format!("'{}'",tok));
        }
    }

    //scope searcher inner to outermost
    fn find_var(&self,name:&str)-> Option<String>{
        for scope in self.scopes.iter().rev(){
            for (n,v) in scope.iter().rev(){
                if n == name{
                    return Some(v.clone());
                }
            }
        }
        None
    }

    fn take_text(&mut self)->String{
        std::mem::take(&mut self.last_text)
    }

     fn is_varname(tok: &str) -> bool {
       !tok.is_empty() && tok.chars().all(|c| c.is_ascii_alphabetic())
   }

   fn parse_list_item(&mut self) {
       self.expect("#GIMMEH ITEM");
       self.emit("<li>");
       self.scopes.push(Vec::new());
       self.parse_variable_define();
       self.parse_inner_list();
       self.expect("#OIC");
       self.scopes.pop();
       self.emit("</li>\n");
   }
}

impl<'a> traits::SyntaxAnalyzer for SyntaxAnalyzer<'a>{
     fn parse_lolcode(&mut self) {
        self.expect("#HAI");
        self.emit("<html>\n");
        while self.cur() == "#OBTW" {
            self.parse_comment();
        }
        self.parse_head();
        self.parse_variable_define(); 
        self.emit("<body>\n");
        self.parse_body();
        self.expect("#KBYE");
        self.emit("</body>\n</html>\n");
    }

    fn parse_head(&mut self) {
        if self.cur() == "#MAEK HEAD" {
            self.advance();
            self.emit("<head>\n");
            self.parse_title();
            self.expect("#MKAY");
            self.emit("</head>\n");
        }
    }

    fn parse_title(&mut self) {
        self.expect("#GIMMEH TITLE");
        self.parse_text();
        self.expect("#OIC");
        let t = self.take_text();
        self.emit(&format!("<title>{}</title>\n", t));
    }

    fn parse_comment(&mut self) {
        self.expect("#OBTW");
        self.parse_text();
        self.expect("#TLDR");
        let t = self.take_text();
        self.emit(&format!("<!-- {} -->\n", t));
    }

    fn parse_body(&mut self) {
        loop {
            let tok = self.cur();
            match tok.as_str() {
                "#KBYE" => break,
                "#MAEK PARAGRAF" => self.parse_paragraph(),
                "#OBTW" => self.parse_comment(),
                "#GIMMEH BOLD" => self.parse_bold(),
                "#MAEK LIST" => self.parse_list(),
                "#GIMMEH LINX" => self.parse_link(),
                "#NEWLINE" => self.parse_newline(),
                "#GIMMEH VAR" => self.parse_variable_use(),
                _ => {
                    if Self::is_text(&tok) {
                        self.parse_text();
                        let t = self.take_text();
                        self.emit(&format!("{} ", t));
                    } else {
                        self.syntax_error("body content or '#KBYE'");
                    }
                }
            }
        }
    }

    fn parse_paragraph(&mut self) {
        self.expect("#MAEK PARAGRAF");
        self.emit("<p>");
        self.scopes.push(Vec::new());
        self.parse_variable_define();
        self.parse_inner_paragraph();
        self.expect("#MKAY");
        self.scopes.pop();
        self.emit("</p>\n");
    }

    //allows for open text while paragraph hasn't been closed
     fn parse_inner_paragraph(&mut self) {
        while self.cur() != "#MKAY" {
            self.parse_inner_text();
        }
    }

    fn parse_inner_text(&mut self) {
        let tok = self.cur();
        match tok.as_str() {
            "#GIMMEH VAR" => self.parse_variable_use(),
            "#OBTW" => self.parse_comment(),
            "#GIMMEH BOLD" => self.parse_bold(),
            "#MAEK LIST" => self.parse_list(),
            "#GIMMEH LINX" => self.parse_link(),
            "#NEWLINE" => self.parse_newline(),
            _ => {
                if Self::is_text(&tok) {
                    self.parse_text();
                    let t = self.take_text();
                    self.emit(&format!("{} ", t));
                } else {
                    self.syntax_error("paragraph content or '#MKAY'");
                }
            }
        }
    }

    fn parse_variable_define(&mut self) {
        if self.cur() == "#IHAZ" {
            self.advance();
            let name = self.cur();
            if !Self::is_varname(&name) {
                self.syntax_error("a variable name (letters only)");
            }
            self.advance();
            self.expect("#ITIZ");
            self.parse_text();
            let value = self.take_text();
            self.expect("#MKAY");
            // goes into the innermost scope
            self.scopes.last_mut().unwrap().push((name, value));
        }
    }

    fn parse_variable_use(&mut self) {
        self.expect("#GIMMEH VAR");
        let name = self.cur();
        if !Self::is_varname(&name) {
            self.syntax_error("a variable name (letters only)");
        }
        self.advance();
        self.expect("#OIC");
        match self.find_var(&name) {
            Some(value) => self.emit(&format!("{} ", value)),
            None => {
                eprintln!("Static semantic error at token #{}: variable '{}' used before it is defined (or out of scope).", self.compiler.lexer.count, name);
                std::process::exit(1);
            }
        }
    }

    fn parse_bold(&mut self) {
        self.expect("#GIMMEH BOLD");
        self.parse_text();
        self.expect("#OIC");
        let t = self.take_text();
        self.emit(&format!("<b>{}</b> ", t));
    }

    fn parse_list(&mut self) {
        self.expect("#MAEK LIST");
        self.emit("<ul>\n");
        self.parse_list_item(); // at least one item is required
        self.parse_list_items();
        self.expect("#MKAY");
        self.emit("</ul>\n");
    }

    fn parse_list_items(&mut self) {
        while self.cur() == "#GIMMEH ITEM" {
            self.parse_list_item();
        }
    }

    fn parse_inner_list(&mut self) {
        while self.cur() != "#OIC" {
            let tok = self.cur();
            match tok.as_str() {
                "#GIMMEH BOLD" => self.parse_bold(),
                "#OBTW" => self.parse_comment(),
                "#GIMMEH LINX" => self.parse_link(),
                "#NEWLINE" => self.parse_newline(),
                "#GIMMEH VAR" => self.parse_variable_use(),
                _ => {
                    if Self::is_text(&tok){
                        self.parse_text();
                        let t = self.take_text();
                        self.emit(&format!("{} ", t));
                    } 
                    else{
                        self.syntax_error("list item content or '#OIC'");
                    }
                }
            }
        }
    }

    fn parse_link(&mut self) {
        self.expect("#GIMMEH LINX");
        let addr = self.cur();
        if !Self::is_text(&addr) {
            self.syntax_error("an address");
        }
        self.advance();
        self.expect("#OIC");
        self.emit(&format!("<a href=\"{}\">{}</a> ", addr, addr));
    }

    fn parse_newline(&mut self) {
        self.expect("#NEWLINE");
        self.emit("<br>\n");
    }

    fn parse_text(&mut self) {
        if !Self::is_text(&self.cur()) {
            self.syntax_error("text");
        }
        let mut words: Vec<String> = Vec::new();
        while Self::is_text(&self.cur()) {
            words.push(self.cur());
            self.advance();
        }
        self.last_text = words.join(" ");
    }
}

fn open_browser(path: &Path) {
    let status = 
    if cfg!(target_os = "windows"){
        Command::new("cmd").args(["/C", "start", ""]).arg(path).status()
    } 
    else if cfg!(target_os = "macos"){
        Command::new("open").arg(path).status()
    } 
    else{
        Command::new("xdg-open").arg(path).status()
    };

    if status.is_err() {
        eprintln!("Could not open the browser. Open {} manually.", path.display());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: lolcompiler <input.lol>");
        std::process::exit(1);
    }
 
    let path = Path::new(&args[1]);
    if path.extension().and_then(|e| e.to_str()) != Some("lol") {
        eprintln!("Usage error: input file must have a .lol extension.");
        std::process::exit(1);
    }
 
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Could not read '{}': {}", args[1], e);
            std::process::exit(1);
        }
    };
 
    let mut compiler = Compiler::new();
    compiler.compile(&source); // exits on any error, so no file is written
 
    let out_path = path.with_extension("html");
    if let Err(e) = fs::write(&out_path, &compiler.html) {
        eprintln!("Could not write '{}': {}", out_path.display(), e);
        std::process::exit(1);
    }
 
    open_browser(&out_path);
}