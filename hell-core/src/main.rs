use std::{
    fs::{self, File},
    io::BufWriter,
};

mod data;
use data::{PaperSize, Project};

use printpdf::{
    IndirectFontRef, Mm, PdfDocument,
    PdfDocumentReference, PdfLayerIndex, PdfLayerReference, PdfPageIndex,
};
use rand::seq::SliceRandom;

// const QUESTION_FONT_SIZE: f64 = 14.0;
//
// fn generate_pdf(project: &Project, output_file: &str) -> anyhow::Result<File> {
//     let (doc, page1, layer1) = PdfDocument::new("", Mm(210.0), Mm(297.0), "Layer 1");
//     let current_layer = doc.get_page(page1).get_layer(layer1);
//
//     #[rustfmt::skip]
//     let monospace = doc.add_external_font(
//         File::open("assets/fonts/monospace.ttf")?
//     )?;
//
//     let mut bruh = 285.0;
//     for question in project.questions.iter() {
//         let mut rng = rand::thread_rng();
//
//         current_layer.begin_text_section();
//
//         current_layer.set_font(&monospace, QUESTION_FONT_SIZE);
//         current_layer.set_line_height(QUESTION_FONT_SIZE);
//         current_layer.set_text_cursor(Mm(5.0), Mm(bruh));
//
//         match question {
//             data::Question::Selection(question) => {
//                 current_layer.write_text(&question.question, &monospace);
//                 current_layer.add_line_break();
//
//                 let mut answers = question.correct.clone();
//                 answers.append(&mut question.incorrect.clone());
//                 answers.shuffle(&mut rng);
//
//                 bruh -= 5.0;
//             }
//             data::Question::Input(_) => {}
//         }
//
//         current_layer.end_text_section();
//     }
//
//     let file = File::create(output_file)?;
//     doc.save(&mut BufWriter::new(&file))?;
//     Ok(file)
// }

#[allow(dead_code)]
pub struct PDFBuilder {
    dimensions: (f64, f64),
    document: PdfDocumentReference,
    pages: Vec<(PdfPageIndex, PdfLayerIndex)>,
    current_page: (PdfPageIndex, PdfLayerIndex),
}

impl PDFBuilder {
    pub fn new(title: &str, paper_size: &PaperSize) -> Self {
        let dimensions = paper_size.get_pdf_size();
        let (document, page1, layer1) =
            PdfDocument::new(title, Mm(dimensions.0), Mm(dimensions.1), "Layer 0");

        let current_layer = document.get_page(page1).get_layer(layer1);
        current_layer.set_text_cursor(Mm(10.0), Mm(dimensions.1 - 10.0));

        Self {
            document,
            dimensions,
            pages: vec![(page1, layer1)],
            current_page: (page1, layer1),
        }
    }

    fn get_layer(&self) -> PdfLayerReference {
        let (page, layer) = self.current_page;
        self.document.get_page(page).get_layer(layer)
    }

    fn get_external_font(&self, path: &str) -> anyhow::Result<IndirectFontRef> {
        Ok(self.document.add_external_font(File::open(path)?)?)
    }

    fn write_text(&self, text: &str, font: &IndirectFontRef, font_size: f64) {
        let current_layer = self.get_layer();
        current_layer.set_font(font, font_size);
        current_layer.set_line_height(font_size);
        current_layer.write_text(text, &font);
    }
    fn write_line(&self, text: &str, font: &IndirectFontRef, font_size: f64) {
        self.write_text(text, font, font_size);
        self.line_break();
    }

    fn draw_input_line(&self, font: &IndirectFontRef, lines: usize) {
        // TODO: this is retarded, it should be fixed
        for _ in 0..lines {
            self.line_break();
            self.write_line(".................................................................................................................", font, 8.0);
            self.line_break();
        }
    }

    fn line_break(&self) {
        let current_layer = self.get_layer();
        current_layer.add_line_break();
    }

    fn save_file(self, path: &str) -> anyhow::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(&file);
        self.document.save(&mut writer)?;
        Ok(())
    }
}

fn main() {
    let project = fs::read_to_string("example.toml").unwrap();
    let project: Project = toml::from_str(&project).unwrap();
    dbg!(&project.settings);

    let pdf_builder = PDFBuilder::new("Test", &project.settings.paper_size);
    let monospace = pdf_builder
        .get_external_font("./assets/fonts/monospace.ttf")
        .unwrap();

    for (qi, question) in project.questions.iter().enumerate() {
        let title = question.get_title();
        let points = question.get_points();
        pdf_builder.write_line(
            &format!("{}. {} ({}pt)", qi + 1, title, points),
            &monospace,
            12.0,
        );

        let mut rng = rand::thread_rng();

        match question {
            data::Question::Selection(question) => {
                let mut questions = question.correct.clone();
                questions.append(&mut question.incorrect.clone());
                questions.shuffle(&mut rng);

                if questions.len() < 4 {
                    println!(
                        "\x1b[33;1mwarning\x1b[0m: on the question: \x1b[3;2m{}\x1b[0m",
                        question.question
                    );
                    println!("         It's recommended have atleast 4 answers!");
                }

                for (i, question) in questions.iter().enumerate() {
                    let letter = char::from_u32(97 + i as u32).unwrap_or('-');
                    pdf_builder.write_line(
                        &format!("  {}) {}", letter, question),
                        &monospace,
                        12.0,
                    );
                }
            }
            data::Question::Input(_) => {
                pdf_builder.draw_input_line(&monospace, 4);
            }
        }

        pdf_builder.line_break();
    }

    pdf_builder.save_file("out.pdf").unwrap();
}
