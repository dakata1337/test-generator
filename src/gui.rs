use crate::{
    data::{OpenedTab, Project},
    pdf_gen::generate_pdf,
    settings::PaperSize,
};
use egui::{ScrollArea, Ui};

pub fn run_gui(project: Project) {
    let options = eframe::NativeOptions::default();

    eframe::run_native("xd", options, Box::new(|_| Box::new(project)))
        .expect("eframe failed to start");
}

impl Project {
    fn draw_topbar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
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

            if ui.button("Generate PDF").clicked() {
                generate_pdf(self);
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
                    });
                    ui.end_row();
                }
            })
            .inner;
    }
    fn draw_configuration(&mut self, ui: &mut Ui) {
        ui.label("Configuration");
        ui.separator();

        ui.label("General settings");
        _ = egui::TextEdit::singleline(&mut self.settings.output).show(ui);

        egui::ComboBox::from_label("Paper size")
            .selected_text(format!("{}", self.settings.paper_size))
            .show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.set_min_width(60.0);
                ui.selectable_value(&mut self.settings.paper_size, PaperSize::A4, "A4");
            });
        ui.end_row();

        ui.add_space(6.0);
        ui.separator();
        ui.add_space(2.0);

        ui.label("Header settings");

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
