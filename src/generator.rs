use std::fs::File;
use std::io::prelude::*;

use crate::lexer;
use crate::quiz::{AnswerType, QuizStruct};

pub fn create_directory(dir: &String) {
    std::fs::create_dir_all(dir).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}

pub fn create_style_sheet(template_file: &String, dir: &String) {
    let mut file = File::create(dir).unwrap();
    let mut template = File::open(template_file).unwrap();
    let mut contents = String::new();
    template.read_to_string(&mut contents).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

pub fn create_html_file(template_file: &String, dir: &String) {
    let mut file = File::create(dir).unwrap();
    let mut template = File::open(template_file).unwrap();
    let mut contents = String::new();
    template.read_to_string(&mut contents).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

fn read_quiz_option(template_file: &String, id: usize, text: &String, answer: bool) -> String {
    let mut file = File::open(template_file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents = contents.replace("{text}", text.as_str());
    contents = contents.replace("{id}", id.to_string().as_str());
    contents = contents.replace("{answer}", answer.to_string().as_str());

    contents
}

fn read_quiz(template_file: &String, id: usize, quiz: &QuizStruct) -> String {
    let mut file = File::open(template_file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents = contents.replace("{id}", id.to_string().as_str());
    contents = contents.replace("{question}", quiz.question.as_str());

    let mut options = "".to_string();
    for i in 0..quiz.answer.len() {
        match &quiz.answer[i] {
            AnswerType::TrueFalse(answer, str) => {
                let option = read_quiz_option(
                    &"templates/quiz_option.html".to_string(),
                    id,
                    str,
                    answer.clone(),
                );
                options = format!("{}{}", options, option);
            }
            _ => {}
        }
    }

    contents = contents.replace("{options}", options.as_str());

    contents
}

pub fn basic_generate_html(template_file: &String, dir: &String, tokens: &Vec<lexer::Token>) {
    let mut html = "".to_string();
    let mut quiz_id = 0;
    for token in tokens {
        match &token.token_type {
            lexer::TokenType::Heading(level) => {
                html = format!("{}<h{}>{}</h{}>", html, level, token.value, level);
            }
            lexer::TokenType::NewLine => {
                html = format!("{}<br>", html);
            }
            lexer::TokenType::Text => {
                html = format!("{}{}", html, token.value);
            }
            lexer::TokenType::EOF => {
                html = format!("{}{}", html, token.value);
            }
            lexer::TokenType::Quiz(quiz) => {
                let quiz = read_quiz(&"templates/quiz.html".to_string(), quiz_id, &quiz);
                html = format!("{}{}", html, quiz);
                quiz_id += 1;
            }
            _ => {
                panic!("Unexpected token type: {:?}", token.token_type);
            }
        }
    }

    let mut file = File::create(dir).unwrap();
    let mut template = File::open(template_file).unwrap();
    let mut contents = String::new();
    template.read_to_string(&mut contents).unwrap();
    contents = contents.replace("{}", html.as_str());
    file.write_all(contents.as_bytes()).unwrap();
}
