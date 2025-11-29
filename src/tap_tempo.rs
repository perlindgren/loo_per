// Tap tempo Widget

use egui::{Color32, Pos2};
use egui::{Key, Sense, Ui};
use epaint::{PathShape, PathStroke};
use std::f32::consts::PI;

pub struct Tempo {
    last_tap: f64,
    last_beat: f64,
    bpm: f64,
}

impl Default for Tempo {
    fn default() -> Self {
        Self {
            last_tap: Default::default(),
            last_beat: Default::default(),
            bpm: 100.0,
        }
    }
}

impl Tempo {
    fn update_on_tap(&mut self, curr_time: f64, diff: f64) {
        println!("Tap {}", curr_time,);

        self.bpm = (60.0 / diff).clamp(40.0, 240.0);

        self.last_tap = curr_time;
        self.last_beat = curr_time;
    }

    pub fn update(&mut self, ui: &mut Ui) {
        let curr_time = ui.input(|input| input.time);
        let diff = curr_time - self.last_tap;

        // ui.label(format!("Now :{:.2}", curr_time));
        // ui.label(format!("Last tap :{:.2}", self.last_tap));
        // ui.label(format!("Bpm :{:06.2}", self.bpm));

        // ui.separator();

        // ui.horizontal(|ui| {
        ui.label("Tap Tempo:");

        // BPM Slider
        ui.add(
            egui::Slider::new(&mut self.bpm, 40.0..=240.0)
                .text("bpm")
                .update_while_editing(false)
                .custom_formatter(|n, _| format!("{n:6.2}")),
        );

        // Tap button
        let desired_size = egui::vec2(80.0, 80.0); // diameter
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
                    self.bpm = (self.bpm + 1.0).min(240.0);
                }
                if input.key_pressed(Key::ArrowDown) {
                    self.bpm = (self.bpm - 1.0).max(40.0);
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

        let progress = (rel_to_next_beat).clamp(0.0, 1.0);
        if to_next_beat <= 0.0 {
            self.last_beat = next_beat;
        }

        // Draw the circle
        let painter = ui.painter();

        let radius = desired_size.x / 2.0;
        painter.circle(
            rect.center(),
            radius,
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

        let start_angle = 1.5 * PI; // Start at the top
        let end_angle = 1.5 * PI + 2.0 * PI * (1.0 - progress) as f32;

        let num_segments = 60;
        let center = rect.center();

        let mut points = Vec::new();
        for i in 0..=num_segments {
            let angle =
                start_angle + (end_angle - start_angle) * (i as f32) / (num_segments as f32);
            let x = center.x + radius * angle.cos();
            let y = center.y + radius * angle.sin();
            points.push(Pos2::new(x, y));
        }

        // Create the path shape for the arc
        let arc_path = PathShape {
            points,
            closed: false, // Important: set to false for an open arc
            fill: Color32::TRANSPARENT,
            stroke: PathStroke {
                width: 2.0,
                color: epaint::ColorMode::Solid(Color32::WHITE),
                kind: egui::StrokeKind::Middle,
            },
        };

        // Paint the shape onto the screen
        painter.add(arc_path);
    }
}
