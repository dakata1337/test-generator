use serde::{Deserialize, Serialize};

use crate::settings::Settings;

const fn default_points() -> u8 {
    1
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SelectionQuestion {
    #[serde(skip)]
    pub question_buf: String,
    pub question: String,
    pub correct: Vec<String>,
    pub incorrect: Vec<String>,
    #[serde(default = "default_points")]
    pub points: u8,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct InputQuestion {
    #[serde(skip)]
    pub question_buf: String,
    pub question: String,
    pub number_of_lines: u16,
    #[serde(default = "default_points")]
    pub points: u8,
}

#[derive(Deserialize, Serialize, Clone)]
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
    pub fn get_title_buf(&mut self) -> &mut String {
        match self {
            Question::Selection(q) => &mut q.question_buf,
            Question::Input(q) => &mut q.question_buf,
        }
    }
    pub fn update_title_from_buf(&mut self) {
        match self {
            Question::Selection(q) => q.question = q.question_buf.clone(),
            Question::Input(q) => q.question = q.question_buf.clone(),
        }
    }
    pub fn update_buf_from_title(&mut self) {
        match self {
            Question::Selection(q) => q.question_buf = q.question.clone(),
            Question::Input(q) => q.question_buf = q.question.clone(),
        }
    }
    pub fn get_points(&self) -> u8 {
        match self {
            Question::Selection(q) => q.points,
            Question::Input(q) => q.points,
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Clone)]
pub struct Header {
    pub title: String,
}
impl Default for Header {
    fn default() -> Self {
        Self {
            title: "Test Header".into(),
        }
    }
}

#[derive(Deserialize, Serialize, Default, PartialEq, Eq, Clone)]
pub enum OpenedTab {
    #[default]
    Questions,
    Configuration,
    Settings,
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct GuiState {
    pub opened_tab: OpenedTab,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Clone)]
pub struct Project {
    pub settings: Settings,
    pub header: Header,
    pub questions: Vec<Question>,
    #[serde(skip)]
    pub gui_state: GuiState,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            settings: Default::default(),
            header: Default::default(),
            questions: Default::default(),
            gui_state: Default::default(),
        }
    }
}
