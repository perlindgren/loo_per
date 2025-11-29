// Tap tempo Widget

use eframe::egui_glow::painter;
use egui::{Key, Sense, Ui};
use std::time::{Duration, SystemTime};

#[derive(Default, Copy, Clone, Debug)]
pub enum Mode {
    #[default]
    Waiting,
    Learning,
}

pub struct Tempo {
    last_tap: f64,
    last_beat: f64,
    bpm: f64,
    mode: Mode,
    bpm_input_text: String,
}

impl Default for Tempo {
    fn default() -> Self {
        Self {
            last_tap: Default::default(),
            last_beat: Default::default(),
            bpm: 100.0,
            mode: Default::default(),
            bpm_input_text: format!("{:06.2}", 100.0),
        }
    }
}

impl Tempo {
    fn update_bpm_text(&mut self) {
        self.bpm_input_text = format!("{:06.2}", self.bpm);
    }

    fn update_on_tap(&mut self, curr_time: f64, diff: f64) {
        println!("pressed {} {:?}", curr_time, self.mode);
        self.mode = Mode::Learning;
        self.bpm = 60.0 / diff;
        self.update_bpm_text();

        self.last_tap = curr_time;
        self.last_beat = curr_time;
    }

    pub fn tap_tempo(&mut self, ui: &mut Ui) {
        let curr_time = ui.input(|input| input.time);
        let diff = curr_time - self.last_tap;
        if diff > 4.0 {
            self.mode = Mode::Waiting;
        }

        ui.label(format!("Now :{:.2}", curr_time));
        ui.label(format!("Last tap :{:.2}", self.last_tap));
        ui.label(format!("Bpm :{:06.2}, Mode {:?}", self.bpm, self.mode));

        let bpm_re =
            ui.add(egui::TextEdit::singleline(&mut self.bpm_input_text).desired_width(40.0));

        if bpm_re.changed() {
            println!("changed to {}", self.bpm_input_text);
        }

        if bpm_re.lost_focus()
            && ui.input(|i| i.key_pressed(Key::Enter))
            && let Ok(new_bpm) = self.bpm_input_text.trim().parse::<f64>()
        {
            self.bpm = new_bpm;
            self.update_bpm_text();
        }

        // Tap button
        let desired_size = egui::vec2(60.0, 60.0); // diameter
        let (rect, _response) = ui.allocate_exact_size(desired_size, Sense::click());

        // Mouse click tap
        ui.input(|input| {
            // Hovering the tap button
            if input
                .pointer
                .hover_pos()
                .map(|p| rect.contains(p))
                .unwrap_or(false)
            {
                // BPM +/-
                if input.key_pressed(Key::ArrowUp) {
                    self.bpm += 1.0;
                    self.update_bpm_text();
                }
                if input.key_pressed(Key::ArrowDown) {
                    self.bpm -= 1.0;
                    self.update_bpm_text();
                }

                // Space/Enter or Mouse tap
                if input.pointer.button_pressed(egui::PointerButton::Primary)
                    || input.key_pressed(Key::Enter)
                    || input.key_pressed(Key::Space)
                {
                    self.update_on_tap(curr_time, diff);
                }
            }
        });

        // Beat
        let next_beat = self.last_beat + 60.0 / self.bpm;
        let to_next_beat = next_beat - curr_time;
        let rel_to_next_beat = to_next_beat / (60.0 / self.bpm);

        // println!(
        //     "next beat at {:06.2} to next {:06.2}",
        //     next_beat, rel_to_next_beat
        // );

        let progress = (rel_to_next_beat).clamp(0.0, 1.0);
        if to_next_beat <= 0.0 {
            self.last_beat = next_beat;
        }

        ui.add(
            egui::ProgressBar::new(progress as f32)
                .show_percentage()
                .text("Time since last tap"),
        );

        // Draw the circle
        let painter = ui.painter();
        painter.circle(
            rect.center(),
            desired_size.x / 2.0,
            ui.visuals().widgets.inactive.bg_fill,
            ui.visuals().widgets.inactive.bg_stroke,
        );

        // Draw text in the center
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "TAP",
            egui::TextStyle::Button.resolve(ui.style()),
            ui.visuals().widgets.inactive.fg_stroke.color,
        );
    }
}
