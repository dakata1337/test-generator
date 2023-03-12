use std::fs;

use clap::Parser;
use data::{InputQuestion, Project, Question};
use druid::{
    widget::{Button, Flex, Label, List, Scroll},
    AppLauncher, Widget, WidgetExt, WindowDesc, Size,
};

pub mod data;

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

fn run_gui(state: Project) {
    let window = WindowDesc::new(build_ui())
        .window_size(Size::new(1280.0, 720.0));

    AppLauncher::with_window(window)
        .log_to_console()
        .launch(state)
        .expect("failed to load gui");
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

    run_gui(state);
}
