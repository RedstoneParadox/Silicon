use std::ops::Deref;
use std::str::Chars;
use crate::lex::Tokens::Equality;
use nom::*;
use nom::IResult;
use std::str;
use std::sync::mpsc::channel;
use crate::lex::LexPattern::Alphanumeric;
use crate::lex::Tokens::PlusAssign;
use crate::lex::Tokens::Assign;
use crate::lex::Tokens::Increment;
use crate::lex::Tokens::Plus;
use crate::lex::Tokens::Whitespace;
use std::str::from_utf8;
use crate::lex::Tokens::IdEquality;
use crate::lex::Tokens::AString;
use crate::lex::Tokens::Word;
use crate::lex::Tokens::Num;

pub fn parse() {

    let mut test_lexer = Lexer::new();
    test_lexer.add_rule(LexRule(LexPattern::String("+".to_string(), true), LexPattern::None, 1));
    test_lexer.add_rule(LexRule(LexPattern::String("++".to_string(), true), LexPattern::None, 2));
    test_lexer.add_rule(LexRule(LexPattern::String("=".to_string(), true), LexPattern::None, 1));
    test_lexer.add_rule(LexRule(LexPattern::String("+=".to_string(), true), LexPattern::None, 2));
    test_lexer.add_rule(LexRule(LexPattern::String("==".to_string(), true), LexPattern::None, 2));
    test_lexer.add_rule(LexRule(LexPattern::String("===".to_string(), true), LexPattern::None, 3));
    test_lexer.add_rule(LexRule(LexPattern::String(" ".to_string(), true), LexPattern::None, 1));
    test_lexer.add_rule(LexRule(LexPattern::String('"'.to_string(), true), LexPattern::String('"'.to_string(), true), 1));
    test_lexer.add_rule(LexRule(LexPattern::Alphabetic(true), LexPattern::Alphanumeric(true), 1));
    test_lexer.add_rule(LexRule(LexPattern::String("/*".to_string(), true), LexPattern::String("*/".to_string().to_string(), true), 1));
    test_lexer.add_rule(LexRule(LexPattern::Numeric(true), LexPattern::Numeric(true), 1));
    let results : Vec<Tokens> = test_lexer.lex(r#"+ = == += ++ === /*This is a comment*/ "hi" aword 1111="#.to_string());

    println!("Test lexer output!");
    for result in results {
        println!("{}", result.to_string())
    }
    println!("End test lexer output!");
}

pub enum Tokens {
    Plus,
    Increment,
    Assign,
    PlusAssign,
    Equality,
    IdEquality,
    AString(String),
    Word(String),
    Num(String),
    Whitespace
}

impl Tokens {

    fn from_lexer(lex : String) -> Option<Tokens> {

        match lex.as_str() {
            "+" => Some(Plus),
            "++" => Some(Increment),
            "=" => Some(Assign),
            "+=" => Some(PlusAssign),
            "==" => Some(Equality),
            "===" => Some(IdEquality),
            " " => None,
            _ => {
                if lex.starts_with('"') {
                    let mut slice = lex.trim_start_matches('"');
                    slice = slice.trim_end_matches('"');
                    Some(AString(slice.to_string()))
                } else if lex.starts_with("/*") {
                    None
                } else {
                    match lex.clone().chars().nth(0) {
                        Some(c) => {
                            if c.is_ascii_alphabetic() {
                                Some(Word(lex))
                            } else if c.is_ascii_digit() {
                                Some(Num(lex))
                            } else {
                                panic!("Unknown Token!")
                            }
                        },
                        None => panic!("Unknown Token!")
                    }
                }
            }
        }
    }
}

impl ToString for Tokens {

    fn to_string(&self) -> String {

        let result = match self {
            Plus => "+".to_string(),
            Increment => "++".to_string(),
            Assign => "=".to_string(),
            PlusAssign => "+=".to_string(),
            Equality => "==".to_string(),
            IdEquality => "===".to_string(),
            Whitespace => " ".to_string(),
            AString(contents) => {
                let mut string_string: String = '"'.to_string();
                string_string.push_str(contents.as_str());
                string_string.push('"');
                string_string
            },
            Word(contents) => contents.clone(),
            Num(contents) => contents.clone(),
            _ => "".to_string()
        };

        return result;
    }
}

#[derive(Clone)]
struct LexRule(LexPattern, LexPattern, usize);

#[derive(Clone)]
pub enum LexPattern {
    String(String, bool),
    Alphabetic(bool),
    Numeric(bool),
    Alphanumeric(bool),
    None
}

struct Lexer {
    rules : Vec<LexRule>,
    count : usize,
    current_token : String,
    chars : Vec<char>
}

impl Lexer {

    fn new() -> Lexer {
        Lexer {rules : vec![], count : 0, current_token : "".to_string(), chars : vec![]}
    }

    fn add_rule(&mut self, rule : LexRule) {
        self.rules.push(rule);
    }

    fn next(&mut self) -> char {

        let mut result: char = ' ';

        if self.count < self.chars.len() {
            result = self.chars[self.count];
        }

        self.count += 1;
        return result
    }

    fn back(&mut self) {
        self.count -= 1;
    }

    fn look_ahead(&mut self, distance : usize) -> Vec<char> {
        let mut peeks: Vec<char> = vec![];

        let count = self.count;

        for i in 0..distance {

            if (self.count + i) < self.chars.len() {
                peeks.push(self.chars[self.count + i].clone())
            }
            else {
                peeks.push(' ')
            }
        }

        return peeks
    }

    fn lex(&mut self, input : String) -> Vec<Tokens> {

        let mut collected_chars : Vec<char> = input.chars().collect();
        &self.chars.append(&mut collected_chars);
        let len : usize = self.chars.len().clone();

        let mut lex_results : Vec<Tokens> = vec![];

        loop {
            let next_char = self.next();
            self.current_token.push(next_char);
            let clone_rules = self.rules.clone();
            let mut rule : &Option<LexRule> = &self.match_rules(clone_rules, 0);

            match &rule {
                Some(a_rule) => match &a_rule.1 {
                    LexPattern::None => {
                        let from_lex : Option<Tokens> = Tokens::from_lexer(self.current_token.clone());

                        match from_lex {
                            Some(tokens) => lex_results.push(tokens),
                            None => {}
                        }

                        self.current_token.clear();
                    },
                    LexPattern::String(pat, should) => {

                        loop {
                            let next_char = self.next();
                            self.current_token.push(next_char);
                            if self.current_token.ends_with(pat.as_str()) {
                                break;
                            }
                        }

                        let from_lex : Option<Tokens> = Tokens::from_lexer(self.current_token.clone());

                        match from_lex {
                            Some(tokens) => lex_results.push(tokens),
                            None => {}
                        }

                        self.current_token.clear();
                    },
                    LexPattern::Alphabetic(should) => {

                        loop {
                            let next_char = self.next();
                            if !(next_char.is_ascii_alphabetic()) {
                                self.back();
                                break;
                            }
                            self.current_token.push(next_char);
                        }

                        let from_lex : Option<Tokens> = Tokens::from_lexer(self.current_token.clone());

                        match from_lex {
                            Some(tokens) => lex_results.push(tokens),
                            None => {}
                        }

                        self.current_token.clear();

                    },
                    LexPattern::Alphanumeric(should) => {

                        loop {
                            let next_char = self.next();
                            if !(next_char.is_ascii_alphanumeric()) {
                                self.back();
                                break;
                            }
                            self.current_token.push(next_char);
                        }

                        let from_lex : Option<Tokens> = Tokens::from_lexer(self.current_token.clone());

                        match from_lex {
                            Some(tokens) => lex_results.push(tokens),
                            None => {}
                        }

                        self.current_token.clear();

                    },
                    LexPattern::Numeric(should) => {

                        loop {
                            let next_char = self.next();
                            if !(next_char.is_ascii_digit()) {
                                self.back();
                                break;
                            }
                            self.current_token.push(next_char);
                        }

                        let from_lex : Option<Tokens> = Tokens::from_lexer(self.current_token.clone());

                        match from_lex {
                            Some(tokens) => lex_results.push(tokens),
                            None => {}
                        }

                        self.current_token.clear();

                    },
                    _ => panic!("We're not there yet!")
                }
                None => {}
            }

            if self.count > self.chars.len() {
                break
            }
        }

        return lex_results;
    }

    fn match_rules(&mut self, rules : Vec<LexRule>, back_num : usize) -> Option<LexRule> {

        let mut matches : Vec<LexRule> = vec![];

        for rule in rules {

            let should_add : bool = match &rule.0 {
                LexPattern::String(pat, should) => {
                    let peek_vec : String = self.look_ahead(pat.len()).into_iter().collect();
                    let mut peek_token = self.current_token.clone();
                    peek_token.push_str(peek_vec.as_str());
                    if peek_token.starts_with(pat.as_str()) { *should } else { !should }
                },
                LexPattern::Alphabetic(should) => match &self.current_token.clone().chars().nth(0) {
                    Some(c) => { if c.is_ascii_alphabetic() { *should } else { !should } },
                    None => { !should }
                },
                LexPattern::Numeric(should) => match &self.current_token.clone().chars().nth(0) {
                    Some(c) => { if c.is_ascii_digit() { *should } else { !should } },
                    None => { !should }
                },
                LexPattern::Alphanumeric(should) => match &self.current_token.clone().chars().nth(0) {
                    Some(c) => { if c.is_ascii_alphanumeric() { *should } else { !should } },
                    None => { !should }
                },
                LexPattern::None => { panic!("Pattern type None should not be used here!") }
            };

            if should_add {
                matches.push(rule.clone())
            }

        }

        let mut longest_match: Option<LexRule> = None;
        let mut longest_len: usize = 0;

        for i in 0..matches.len() {

            if matches[i].2 > longest_len {
                longest_len = matches[i].2;
                longest_match = Some(matches[i].clone());
            }
        }

        for i in 1..longest_len {
            let next_token = self.next();
            self.current_token.push(next_token)
        }
        return longest_match
    }
}