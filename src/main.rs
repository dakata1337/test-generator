use std::fs;

use clap::Parser;
use data::{InputQuestion, Project, Question};
use druid::{
    widget::{Button, Flex, Label, List, Scroll},
    AppLauncher, Size, Widget, WidgetExt, WindowDesc,
};
use pdf_gen::generate_pdf;

pub mod data;
pub mod pdf_elements;
pub mod pdf_gen;
pub mod settings;

#[derive(Parser)]
struct Args {
    path: Option<String>,
}

fn build_ui() -> impl Widget<Project> {
    let mut root = Flex::column();

    root.add_child(Button::new("Add New").on_click(|_, data: &mut Project, _| {
        #[rustfmt::skip]
        data.questions.push_back(
            Question::Input(InputQuestion::new(
                "Question here...".into(),
                1,
                1,
            ))
        );
    }));

    root.add_child(
        Button::new("Generate PDF").on_click(|_, data: &mut Project, _| {
            println!("Generating: \x1b[1m{}\x1b[0m", data.settings.output);
            let time = generate_pdf(data, &data.settings.output);
            println!("Generating took: {:?}", time);
        }),
    );

    #[rustfmt::skip]
    root.add_child(
        Scroll::new(
            List::new(|| {
                Label::new(|item: &Question, _env: &_|
                    format!("{}", item.get_title()))
            })
            .lens(Project::questions),
        )
    );

    root
}

fn main() {
    let args = Args::parse();

    let state = match args.path {
        Some(path) => {
            let content = fs::read_to_string(path).unwrap();
            toml::from_str(&content).unwrap()
        }
        None => Project::default(),
    };

    let window = WindowDesc::new(build_ui()).window_size(Size::new(1280.0, 720.0));

    AppLauncher::with_window(window)
        .log_to_console()
        .launch(state)
        .expect("failed to load gui");
}
