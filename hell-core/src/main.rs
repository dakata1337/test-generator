use std::{fs, time::Instant};

use rand::seq::SliceRandom;
use rckive_genpdf::{
    elements::{Break, PaddedElement, Text},
    Document, Margins,
};

mod data;
use data::{Project, Question};

mod elements;
use elements::{AlphabeticOrderedList, DottedLine};

fn gen_questions(doc: &mut Document, project: &Project) {
    let mut rng = rand::thread_rng();
    let language = project.settings.language.clone();

    for (i, question) in project.questions.iter().enumerate() {
        let title = question.get_title();
        let title = format!("{}. {}", i + 1, title);

        match question {
            Question::Selection(question) => {
                let title = match question {
                    q if q.correct.len() + q.incorrect.len() > 16 => {
                        panic!("Question #{} has too many answers!", i + 1)
                    }
                    q if q.correct.len() > 1 => {
                        format!("{} ({})", title, language.multiple_answers_hint())
                    }
                    _ => title,
                };
                doc.push(Text::new(title));

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
                doc.push(Text::new(title));
                doc.push(Break::new(0.5));
                for _ in 0..question.number_of_lines {
                    #[rustfmt::skip]
                    doc.push(PaddedElement::new(
                        DottedLine,
                        Margins::vh(1.5, 0.0)
                    ));
                }
            }
        }

        doc.push(Break::new(1));
    }
}

fn main() {
    let start = Instant::now();

    let project = fs::read_to_string("example.toml").unwrap();
    let project: Project = toml::from_str(&project).unwrap();

    let font_family = rckive_genpdf::fonts::from_files("./assets/fonts", "monospace", None)
        .expect("Failed to load font family");

    let mut doc = rckive_genpdf::Document::new(font_family);
    doc.set_paper_size(project.settings.paper_size);
    doc.set_title("Demo document");

    let mut decorator = rckive_genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    gen_questions(&mut doc, &project);

    doc.render_to_file("output.pdf")
        .expect("Failed to write PDF file");

    println!("Pdf generation took {:?}", start.elapsed());
}
