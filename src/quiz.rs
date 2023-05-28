#[derive(Debug, PartialEq, Clone)]
pub enum AnswerType {
    TrueFalse(bool, String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct QuizStruct {
    pub question: String,
    pub answer: Vec<AnswerType>,
}
