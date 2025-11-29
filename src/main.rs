#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use loo_per::bars::Bars;
use loo_per::meter::*;
use loo_per::tap_tempo::*;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 640.0]),
        ..Default::default()
    };
    eframe::run_native(
        "LooPer with Tap Tempo",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    tempo: Tempo,
    meter: Meter,
    bars: Bars,
}

#[allow(clippy::derivable_impls)]
impl Default for MyApp {
    fn default() -> Self {
        Self {
            tempo: Tempo::default(),
            meter: Meter::default(),
            bars: Bars::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.heading("LooPer with Tap Tempo");
            self.bars.update(ui);
            self.meter.update(ui);
            self.tempo.update(ui);
        });
    }
}
