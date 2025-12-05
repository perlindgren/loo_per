#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use anyhow::{Result, anyhow};
use eframe::egui;
use loo_per::config::*;
use midir::{Ignore, MidiInput};

fn main() -> Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let config = Config::load_from_file("config.toml").unwrap_or_default();

    let mut midi_in = MidiInput::new("midir midi in")?;
    midi_in.ignore(Ignore::None);

    if let Some(id) = config.opt_device {
        let midi_in_port = midi_in.find_port_by_id(id);
        let _conn_in = midi_in.connect(
            &midi_in_port,
            "midir-read-input",
            move |stamp, message, _| {
                println!("{}: {:?} (len = {})", stamp, message, message.len());
            },
            (),
        )?;
    }

    // let midi_in_port =  midi_in.find_port_by_id

    // )?;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 640.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Config Tester",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(App::new(config)))
        }),
    )
    .map_err(|e| anyhow!(" {}", e))
}

struct App {
    config: Config,
}

impl App {
    fn new(config: Config) -> Self {
        Self { config }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        egui::CentralPanel::default().show(ctx, |ui| ui.label(format!("{:?}", self.config)));
    }
}
