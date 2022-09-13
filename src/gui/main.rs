use arboard;
use eframe::*;
use egui::TextBuffer;
use egui_toast;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::fmt::Display;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{cell::Cell, collections::HashMap};

use crate::lib::config::{self, Config};
use crate::lib::util;
use crate::util::Chars;

pub fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(Gui::new(cc))),
    );
}

#[derive()]
struct Gui {
    config: Arc<Mutex<Config>>,
    generated_pass: String,
    char_checks: [(String, bool, Chars); 4],
}

impl Gui {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        println!("Creating GUI");
        let mut config = Arc::new(Mutex::new(Config::default()));
        let mut selected_chars = RefCell::new(vec![false, false, false, false]);
        let mut checkboxes: [(String, bool, Chars); 4] = [
            ("Uppercase".to_string(), true, Chars::Uppercase),
            ("Lowercase".to_string(), true, Chars::Lowercase),
            ("Digits".to_string(), false, Chars::Digits),
            ("Special".to_string(), false, Chars::Special),
        ];
        Gui {
            config,
            generated_pass: util::generate(&config::Config::default()),
            char_checks: checkboxes,
        }
    }

    fn regenerate(&mut self) {
        self.generated_pass = util::generate(&self.config.lock().unwrap());
    }

    fn set_config(&self, config: Config) {
        *self.config.lock().unwrap() = config;
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut toasts = egui_toast::Toasts::new(ctx)
                .anchor((10.0, 10.0))
                .direction(egui::Direction::TopDown);
            ui.add(egui::Slider::new(
                &mut self.config.lock().unwrap().len,
                0..=128,
            ));

            for (n, v, c) in self.char_checks.iter_mut() {
                ui.checkbox(v, n.as_str());
                let bruh = self.config.clone();
                let mut config = &mut (*bruh.lock().unwrap());
                if v.to_owned() {
                    if !config.chars.contains(c) {
                        config.chars.push(c.clone());
                    }
                } else {
                    config.chars.retain(|x| x.clone() != c.clone());
                }
            }

            if ui.button("GENERATE").clicked() {
                ui.output().cursor_icon = egui::CursorIcon::Progress;
                self.regenerate();
            }

            if ui.button("copy to clipboard").clicked() {
                let mut cl = arboard::Clipboard::new().unwrap();
                cl.set_text(self.generated_pass.clone());
                toasts.success("Copied to clipboard", Duration::from_secs(3));
            }

            ui.label(self.generated_pass.as_str());
            ui.label(format!("{}", self.config.lock().unwrap()));
            toasts.show();
        });
    }
}
