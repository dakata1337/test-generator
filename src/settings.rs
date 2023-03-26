use std::fmt::Display;

use rckive_genpdf::Size;
use serde::{Deserialize, Serialize};

const fn default_bool_true() -> bool {
    true
}

#[allow(dead_code)]
#[derive(Default, Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
pub enum PaperSize {
    #[default]
    A4,
}
impl Display for PaperSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            PaperSize::A4 => "A4",
        };
        write!(f, "{}", name)
    }
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
#[derive(Deserialize, Serialize, Clone)]
pub struct Settings {
    #[serde(default = "default_bool_true")]
    pub show_hints: bool,
    pub paper_size: PaperSize,
    #[serde(default)]
    pub language: Language,
    pub fonts_path: String,
    pub font: String,
    pub output: String,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            show_hints: Default::default(),
            paper_size: Default::default(),
            language: Default::default(),
            fonts_path: Default::default(),
            font: Default::default(),
            output: "out.pdf".into(),
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, PartialEq, Eq, Clone)]
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
    #[inline]
    pub fn input_class_num(&self) -> &str {
        match self {
            Language::English => "No",
            Language::Bulgarian => "№",
        }
    }
    #[inline]
    pub fn get_name(&self) -> &str {
        match self {
            Language::English => "English",
            Language::Bulgarian => "Български",
        }
    }
}
