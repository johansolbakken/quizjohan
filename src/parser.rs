use crate::{lexer, quiz};

pub fn parse(tokens: &mut Vec<lexer::Token>) {
    // if meet heading, take contents of next text token and put as value of heading token and delete the text token
    let mut new_tokens: Vec<lexer::Token> = Vec::new();
    let mut index = 0;
    while index < tokens.len() {
        let token = tokens.get(index).unwrap();
        match token.token_type {
            lexer::TokenType::Heading(_) => {
                let mut heading = token.clone();
                let text = tokens.get(index + 1).unwrap().clone();
                heading.value = text.value.clone();
                new_tokens.push((heading).clone());
                index += 2;
            }
            lexer::TokenType::Text => {
                if token.value == "[quiz" {
                    let mut quiz = quiz::QuizStruct {
                        question: String::new(),
                        answer: Vec::new(),
                    };
                    // ? question
                    // + true answer
                    // - false answer
                    // until ]

                    let mut answer = tokens.get(index).unwrap().clone();
                    while answer.value != "]" {
                        match answer.value.chars().nth(0).unwrap() {
                            '?' => {
                                quiz.question = answer.value.chars().skip(1).collect();
                            }
                            '+' => {
                                let true_answer = quiz::AnswerType::TrueFalse(
                                    true,
                                    answer.value.chars().skip(1).collect(),
                                );
                                quiz.answer.push(true_answer);
                            }
                            '-' => {
                                let false_answer = quiz::AnswerType::TrueFalse(
                                    false,
                                    answer.value.chars().skip(1).collect(),
                                );
                                quiz.answer.push(false_answer);
                            }
                            _ => {}
                        }
                        index += 1;

                        answer = tokens.get(index).unwrap().clone();
                    }

                    let mut quiz_token = lexer::Token {
                        token_type: lexer::TokenType::Quiz(quiz),
                        value: String::new(),
                    };
                    println!("{:?}", quiz_token);
                    new_tokens.push(quiz_token);
                    index += 1;
                } else {
                    new_tokens.push((*token).clone());
                    index += 1;
                }
            }
            _ => {
                new_tokens.push((*token).clone());
                index += 1;
            }
        }
    }
    *tokens = new_tokens;
}
