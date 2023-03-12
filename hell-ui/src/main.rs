use druid::{
    im::{self, Vector},
    widget::{Button, Flex, Label, List, Scroll},
    AppLauncher, Data, Lens, LensExt, Size, Widget, WindowDesc, WidgetExt,
};

#[derive(Debug, Data, Clone, PartialEq, Eq)]
enum Question {
    Selection,
    Input,
}

#[derive(Debug, Data, Lens, Clone)]
struct UIState {
    questions: im::Vector<Question>,
}

#[derive(Debug, Data, Lens, Clone)]
struct AppState {
    ui: UIState,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            ui: UIState {
                questions: Vector::new(),
            },
        }
    }
}

fn build_ui() -> impl Widget<AppState> {
    let mut root = Flex::column();

    root.add_child(
        List::new(|| {
            Label::new(|item: &Question, _env: &_| {
                format!("{:?}", item)
            })
        })
        .lens(AppState::ui.then(UIState::questions))
    );

    root.add_child(
        Button::new("New")
            .on_click(|_, data: &mut AppState, _| data.ui.questions.push_back(Question::Input)),
    );

    root
}

fn main() {
    let window = WindowDesc::new(build_ui())
        .title("Test Generator")
        .with_min_size(Size::new(1280.0, 720.0));

    let state = AppState::default();

    AppLauncher::with_window(window)
        .log_to_console()
        .launch(state)
        .expect("failed to launch ui");
}
