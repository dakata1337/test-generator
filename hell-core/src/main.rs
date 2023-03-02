use std::{fs, time::Instant};

mod data;
use data::{Project, Question};
use rand::seq::SliceRandom;
use rckive_genpdf::{
    elements::{Break, OrderedList, PaddedElement, Text},
    Document, Element, Margins, Position, RenderResult,
};

struct DottedLine;
impl Element for DottedLine {
    fn render(
        &mut self,
        context: &rckive_genpdf::Context,
        area: rckive_genpdf::render::Area<'_>,
        style: rckive_genpdf::style::Style,
    ) -> Result<rckive_genpdf::RenderResult, rckive_genpdf::error::Error> {
        let mut result = RenderResult::default();

        let width_per_ch = style.char_width(&context.font_cache, '.');
        let n_chars = area.size().width / f64::from(width_per_ch);

        area.print_str(
            &context.font_cache,
            Position::default(),
            style,
            ".".repeat(f64::from(n_chars) as usize),
        )?;

        let line_height = style.line_height(&context.font_cache);
        result.size.height = line_height;
        Ok(result)
    }
}

fn gen_questions(doc: &mut Document, project: &Project) {
    for (i, question) in project.questions.iter().enumerate() {
        let title = question.get_title();
        doc.push(Text::new(format!("{}. {}", i+1, title)));

        let mut rng = rand::thread_rng();

        // TODO: check if `show_hints` is enabled and print a warning
        // about questions with multiple answers
        match question {
            Question::Selection(question) => {
                let mut questions = question.correct.clone();
                questions.append(&mut question.incorrect.clone());
                questions.shuffle(&mut rng);

                let mut list = OrderedList::new();
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
