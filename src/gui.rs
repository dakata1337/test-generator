use std::{fs::File, io::Write};

use crate::{
    data::{OpenedTab, Project, Question},
    pdf_gen::generate_pdf,
    settings::PaperSize,
};
use egui::{ScrollArea, TextStyle, Ui};

pub fn run_gui(project: Project) {
    let options = eframe::NativeOptions::default();

    eframe::run_native("xd", options, Box::new(|_| Box::new(project)))
        .expect("eframe failed to start");
}

fn add_label(label: &str, ui: &mut Ui) {
    ui.add_space(6.0);
    ui.separator();
    ui.add_space(2.0);

    let og = ui.style().clone();
    ui.style_mut().override_text_style = Some(TextStyle::Heading);
    ui.label(label);
    ui.set_style(og);

    ui.add_space(4.0);
}

fn add_answers(label: &str, answers: &mut Vec<String>, ui: &mut Ui) {
    ui.label(label);
    ui.vertical(|ui| {
        for i in 0..answers.len() {
            ui.horizontal(|ui| {
                if let Some(q) = answers.get_mut(i) {
                    _ = egui::TextEdit::singleline(q).show(ui);
                }

                if ui.button("Remove").clicked() {
                    answers.remove(i);
                }
            });
        }
        if ui.button("Add").clicked() {
            answers.push("New Answer".to_string());
        }
    });
}

impl Project {
    fn draw_topbar(&mut self, ui: &mut Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.selectable_value(
                &mut self.gui_state.opened_tab,
                OpenedTab::Questions,
                "Questions",
            );
            ui.selectable_value(
                &mut self.gui_state.opened_tab,
                OpenedTab::Configuration,
                "Configuration",
            );
            ui.selectable_value(
                &mut self.gui_state.opened_tab,
                OpenedTab::Settings,
                "Settings",
            );

            ui.separator();

            if ui.button("Generate PDF").clicked() {
                generate_pdf(self);
            }
            if ui.button("Save Project").clicked() {
                let mut project_file = File::create("project.toml").unwrap();
                let str = toml::to_string(self).unwrap();
                _ = project_file.write(str.as_bytes());
            }
            if ui.button("Open Project").clicked() {
                let content = std::fs::read_to_string("project.toml").unwrap();
                *self = toml::from_str(&content).unwrap();
                for q in self.questions.iter_mut() {
                    q.update_buf_from_title();
                }
            }
        });
        ui.end_row();
    }

    fn draw_questions(&mut self, ui: &mut Ui) {
        ui.label("Questions");
        let scroll_area = ScrollArea::vertical().auto_shrink([false; 2]);

        _ = scroll_area
            .show(ui, |ui| {
                for q in self.questions.iter_mut() {
                    ui.separator();
                    ui.collapsing(q.get_title(), |ui| {
                        _ = egui::TextEdit::multiline(q.get_title_buf()).show(ui);

                        if ui.button("Update").clicked() {
                            q.update_title_from_buf();
                        }
                        match q {
                            Question::Selection(q) => {
                                add_answers("Correct answers:", &mut q.correct, ui);
                                add_answers("Incorrect answers:", &mut q.incorrect, ui);
                            }
                            Question::Input(q) => {
                                ui.add(egui::Slider::new(&mut q.number_of_lines, 0..=64))
                                    .on_hover_text("How many lines of text to be generated");
                            }
                        }
                    });
                    ui.end_row();
                }
            })
            .inner;
    }
    fn draw_configuration(&mut self, ui: &mut Ui) {
        add_label("General settings", ui);
        _ = egui::TextEdit::singleline(&mut self.settings.output).show(ui);

        egui::ComboBox::from_label("Paper size")
            .selected_text(format!("{}", self.settings.paper_size))
            .show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.set_min_width(60.0);
                ui.selectable_value(&mut self.settings.paper_size, PaperSize::A4, "A4");
            });

        ui.label("Fonts path: ");
        egui::TextEdit::singleline(&mut self.settings.fonts_path).show(ui);

        ui.label("Font: ");
        egui::TextEdit::singleline(&mut self.settings.font).show(ui);
        ui.end_row();

        add_label("Header settings", ui);

        ui.label("Header title: ");
        egui::TextEdit::singleline(&mut self.header.title).show(ui);
        _ = ui.end_row();
    }
    fn draw_settings(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("egui theme:");
            egui::widgets::global_dark_light_mode_buttons(ui);
        });
    }
}

impl eframe::App for Project {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_topbar(ui);

            match self.gui_state.opened_tab {
                OpenedTab::Questions => self.draw_questions(ui),
                OpenedTab::Configuration => self.draw_configuration(ui),
                OpenedTab::Settings => self.draw_settings(ui),
            }
        });
    }
}
