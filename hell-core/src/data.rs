use rckive_genpdf::Size;
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
    pub number_of_lines: u16,
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

    pub fn get_points(&self) -> &u8 {
        match self {
            Question::Selection(q) => &q.points,
            Question::Input(q) => &q.points,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone, Copy)]
pub enum PaperSize {
    A4,
}
impl Into<Size> for PaperSize {
    fn into(self) -> Size {
        let (w, h) = match self {
            PaperSize::A4 => (210.0, 297.0),
        };

        Size {
            width: w.into(),
            height: h.into(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone, Copy)]
pub enum Language {
    English,
    Bulgarian,
}
impl Default for Language {
    fn default() -> Self {
        Self::English
    }
}
impl Language {
    #[inline]
    pub fn get_first_char(&self) -> char {
        match self {
            Language::English => 'a',
            Language::Bulgarian => 'а',
        }
    }
    #[inline]
    pub fn multiple_answers_hint(&self) -> &str {
        match self {
            Language::English => "Multiple answers",
            Language::Bulgarian => "Повече от 1 верен отговор",
        }
    }
    #[inline]
    pub fn format_points(&self, points: u8) -> String {
        match self {
            Language::English => format!("__/{points}pt"),
            Language::Bulgarian => format!("__/{points}т"),
        }
    }
    #[inline]
    pub fn input_name(&self) -> &str {
        match self {
            Language::English => "Name",
            Language::Bulgarian => "Име",
        }
    }

    #[inline]
    pub fn input_class(&self) -> &str {
        match self {
            Language::English => "Class",
            Language::Bulgarian => "Клас",
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(default = "default_bool_true")]
    pub show_hints: bool,
    pub paper_size: PaperSize,
    #[serde(default)]
    pub language: Language,
    pub fonts_path: String,
    pub font: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Header {
    pub title: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Project {
    pub settings: Settings,
    pub header: Header,
    pub questions: Vec<Question>,
}
