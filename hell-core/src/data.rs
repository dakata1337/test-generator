use serde::Deserialize;

const fn default_points() -> u8 {
    1
}
const fn default_bool_true() -> bool {
    true
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct SelectionQuestion {
    pub question: String,
    pub correct: Vec<String>,
    pub incorrect: Vec<String>,
    #[serde(default = "default_points")]
    pub points: u8,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct InputQuestion {
    pub question: String,
    #[serde(default = "default_points")]
    pub points: u8,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Question {
    Selection(SelectionQuestion),
    Input(InputQuestion),
}
impl Question {
    pub fn get_title(&self) -> &String {
        match self {
            Question::Selection(q) => &q.question,
            Question::Input(q) => &q.question,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub enum PaperSize {
    A4,
}
impl PaperSize {
    pub fn get_pdf_size(&self) -> (f64, f64) {
        match self {
            PaperSize::A4 => (210.0, 297.0),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(default = "default_bool_true")]
    pub show_hints: bool,
    pub paper_size: PaperSize,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Project {
    pub settings: Settings,
    pub questions: Vec<Question>,
}
