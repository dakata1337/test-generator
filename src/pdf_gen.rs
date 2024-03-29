use std::time::{Duration, Instant};

use ilog::IntLog;
use rand::seq::SliceRandom;
use rckive_genpdf::{
    elements::{Break, PaddedElement, Paragraph, Text},
    style::{Style, StyledString},
    Document, Element, Margins,
};

use crate::{
    data::{Project, Question},
    pdf_elements::{AlphabeticOrderedList, CharRepeat, SplitElement},
};

fn gen_points_element(i: usize, question: &Question, project: &Project) -> impl Element {
    let show_hint = if let Question::Selection(q) = question {
        q.correct.len() >= 2 && project.settings.show_hints
    } else {
        false
    };

    let language = &project.settings.language;

    let title = if show_hint {
        format!(
            "{}. {} ({})",
            i + 1,
            question.get_title(),
            language.multiple_answers_hint()
        )
    } else {
        format!("{}. {}", i + 1, question.get_title())
    };

    let mut points_element =
        Paragraph::new(format!("{}", language.format_points(question.get_points())));
    points_element.set_alignment(rckive_genpdf::Alignment::Right);

    SplitElement::new(Paragraph::new(title), points_element, 0.9)
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
    let name = Paragraph::new(StyledString::new(
        format!(
            "{}: ________________________________________",
            language.input_name()
        ),
        Style::new().with_font_size(14),
    ));

    let class = SplitElement::new(
        Paragraph::new(StyledString::new(
            format!("{}: _____", language.input_class()),
            Style::new().with_font_size(14),
        )),
        Paragraph::new(StyledString::new(
            format!("{}: _____", language.input_class_num()),
            Style::new().with_font_size(14),
        )),
        0.5,
    );

    doc.push(SplitElement::new(name, class, 0.7));

    doc.push(Break::new(0.5));
}

fn gen_questions(doc: &mut Document, project: &Project) -> usize {
    doc.set_font_size(12);
    let mut rng = rand::thread_rng();
    let mut points: usize = 0;
    let language = project.settings.language.clone();

    let mut questions = project.questions.clone();
    if project.settings.randomize_questions {
        questions.shuffle(&mut rng);
    }

    for (i, question) in questions
        .iter()
        .take(project.settings.max_questions as usize)
        .enumerate()
    {
        doc.push(gen_points_element(i, question, &project));
        points += question.get_points() as usize;

        match question {
            Question::Selection(question) => {
                let mut questions: Vec<String> = question.correct.clone().into_iter().collect();
                questions.append(&mut question.incorrect.clone().into_iter().collect());
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

    points
}

fn gen_footer(doc: &mut Document, project: &Project, max_points: usize) {
    let poins_needed_space = max_points.log10() + 2;

    let examiner = SplitElement::new(
        Paragraph::new(format!("{}: ", project.settings.language.get_examiner())),
        Paragraph::new("______________________________"),
        0.0,
    );
    let points = Paragraph::new(format!(
        "{}: {}/{}",
        project.settings.language.get_points_sum(),
        "_".repeat(poins_needed_space as usize),
        max_points
    ));

    doc.push(SplitElement::new(examiner, points, 0.7));
}

pub fn generate_pdf(project: &Project) -> anyhow::Result<Duration> {
    let start = Instant::now();

    let font_family = rckive_genpdf::fonts::from_files(
        &project.settings.fonts_path,
        &project.settings.font,
        None,
    )?;

    let mut doc = rckive_genpdf::Document::new(font_family);
    doc.set_paper_size(project.settings.paper_size);
    doc.set_title(&project.header.title);

    let mut decorator = rckive_genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    gen_header(&mut doc, &project);
    let max_points = gen_questions(&mut doc, &project);
    gen_footer(&mut doc, &project, max_points);
    // TODO: Docs:
    // Разработка на софтуер - генерално
    // Agile, SCRUM
    // Жинен цикъл
    //
    // Изисквания
    // Подобни проекти

    doc.render_to_file(&project.settings.output)?;

    Ok(start.elapsed())
}
