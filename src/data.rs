use druid::{im::Vector, Data, Lens};
use serde::Deserialize;

const fn default_points() -> u8 {
    1
}

#[derive(Data, Deserialize, Clone)]
pub struct SelectionQuestion {
    pub question: String,
    pub correct: Vector<String>,
    pub incorrect: Vector<String>,
    #[serde(default = "default_points")]
    pub points: u8,
}

#[derive(Data, Deserialize, Clone)]
pub struct InputQuestion {
    pub question: String,
    pub number_of_lines: u16,
    #[serde(default = "default_points")]
    pub points: u8,
}

impl InputQuestion {
    pub fn new(question: String, number_of_lines: u16, points: u8) -> Self {
        Self {
            question,
            number_of_lines,
            points,
        }
    }
}

#[derive(Data, Deserialize, Clone)]
#[serde(untagged)]
pub enum Question {
    Selection(SelectionQuestion),
    Input(InputQuestion),
}
impl Question {
    pub fn get_title(&self) -> String {
        match self {
            Question::Selection(q) => q.question.clone(),
            Question::Input(q) => q.question.clone(),
        }
    }
}

#[allow(dead_code)]
#[derive(Data, Lens, Deserialize, Clone)]
pub struct Project {
    pub questions: Vector<Question>,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            questions: Default::default(),
        }
    }
}
