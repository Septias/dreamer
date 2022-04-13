use egui::{Context, Frame, RichText, SidePanel, Window};
use log::warn;

use crate::{
    state::{AppState, Command},
    ACCENT_COLOR, ACCENT_COLOR_STRONG,
};

// TODO:
// - Formvalidation
#[derive(Debug, Default)]
pub struct AddAccount {
    email: String,
    password: String,
}

impl AddAccount {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn ui(&mut self, ctx: &Context, state: &AppState) {
        warn!("hi");
        Window::new("PopUp").show(ctx, |ui| {
            ui.label(RichText::new("Login").size(25.).color(*ACCENT_COLOR));
            ui.label("Username");
            ui.text_edit_singleline(&mut self.email);
            ui.label("Password");
            ui.text_edit_singleline(&mut self.password);
            if ui.button("Login").clicked() {
                state.send_command(Command::Login(self.email.clone(), self.password.clone()));
            }
            //ui.label(RichText::new("Register").size(25.).color(*ACCENT_COLOR));
        });
    }
}
