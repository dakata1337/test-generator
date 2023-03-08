#![allow(incomplete_features)]
#![feature(inherent_associated_types)]

use std::{fs, time::Instant};

use rand::seq::SliceRandom;
use rckive_genpdf::{
    elements::{Break, PaddedElement, Paragraph, Text},
    style::{Style, StyledString},
    Document, Element, Margins,
};

mod data;
use data::{Language, Project, Question};

mod elements;
use elements::{AlphabeticOrderedList, CharRepeat, SplitElement};

trait Punctuation {
    fn ends_with_punctuation(&self) -> bool;
}
impl Punctuation for String {
    fn ends_with_punctuation(&self) -> bool {
        match self.chars().last() {
            Some(ch) => matches!(ch, '?' | '!' | '.'),
            None => false,
        }
    }
}

fn gen_points_element(i: usize, question: &Question, language: &Language) -> impl Element {
    let title = format!("{}. {}", i + 1, question.get_title());

    let mut points_element = Paragraph::new(format!(
        "{}",
        language.format_points(*question.get_points())
    ));
    points_element.set_alignment(rckive_genpdf::Alignment::Right);

    SplitElement::new(
        Box::new(Paragraph::new(title)),
        Box::new(points_element),
        0.9,
    )
}

fn gen_header(doc: &mut Document, project: &Project) {
    let header = &project.header;
    let language = &project.settings.language;

    let mut title = Paragraph::new(StyledString::new(
        &header.title,
        Style::new().with_font_size(18),
    ));
    title.set_alignment(rckive_genpdf::Alignment::Center);
    doc.push(title);

    doc.push(Break::new(1.0));

    // TODO: export to an Element that requires a string and repeats a char until the end of the
    // area
    let name = Box::new(Paragraph::new(StyledString::new(
        format!(
            "{}: ________________________________________",
            language.input_name()
        ),
        Style::new().with_font_size(14),
    )));

    let class = Box::new(Paragraph::new(StyledString::new(
        format!("{}: _____", language.input_class()),
        Style::new().with_font_size(14),
    )));
    doc.push(SplitElement::new(name, class, 0.75));

    doc.push(Break::new(0.5));
}

fn gen_questions(doc: &mut Document, project: &Project) {
    doc.set_font_size(12);
    let mut rng = rand::thread_rng();
    let language = project.settings.language.clone();

    for (i, question) in project.questions.iter().enumerate() {
        doc.push(gen_points_element(i, question, &language));

        match question {
            Question::Selection(question) => {
                let mut questions = question.correct.clone();
                questions.append(&mut question.incorrect.clone());
                questions.shuffle(&mut rng);

                let mut list = AlphabeticOrderedList::new(language.get_first_char());
                for answer in questions {
                    list.push(Text::new(answer))
                }
                doc.push(list);
            }
            Question::Input(question) => {
                doc.push(Break::new(0.5));
                for _ in 0..question.number_of_lines {
                    #[rustfmt::skip]
                    doc.push(PaddedElement::new(
                        CharRepeat::new('.'),
                        Margins::vh(1.5, 0.0)
                    ));
                }
            }
        }

        doc.push(Break::new(1));
    }
}

// TODO: format questions strings (using markdown syntax)
// TODO: create header, title, test subject
// TODO: create student fields for: name, class etc.
fn main() {
    let start = Instant::now();

    let project = fs::read_to_string("example.toml").unwrap();
    let project: Project = toml::from_str(&project).unwrap();

    let font_family = rckive_genpdf::fonts::from_files(
        &project.settings.fonts_path,
        &project.settings.font,
        None,
    )
    .expect("Failed to load font family");

    let mut doc = rckive_genpdf::Document::new(font_family);
    doc.set_paper_size(project.settings.paper_size);
    doc.set_title("Demo document");

    let mut decorator = rckive_genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    gen_header(&mut doc, &project);
    gen_questions(&mut doc, &project);

    doc.render_to_file("output.pdf")
        .expect("Failed to write PDF file");

    print!("\x1b[1mPdf generation took: ");
    match start.elapsed().as_millis() {
        time if time < 12 => print!("\x1b[32m"),
        time if time < 16 => print!("\x1b[33m"),
        _ => print!("\x1b[31m"),
    }
    println!("{:?}", start.elapsed());
    print!("\x1b[0m");
}
