// Meter

use egui::{Key, Sense, Ui};

pub struct Meter {
    upper: u8,
    lower: u8,
}

impl Default for Meter {
    fn default() -> Self {
        Self { upper: 4, lower: 4 }
    }
}

impl Meter {
    pub fn update(&mut self, ui: &mut Ui) {
        ui.label(format!("Meter: {}/{}", self.upper, self.lower));

        // Upper Slider
        ui.add(
            egui::Slider::new(&mut self.upper, 1..=16)
                .text("Upper")
                .update_while_editing(false),
        );

        // Lower Slider
        ui.add(
            egui::Slider::new(&mut self.lower, 1..=16)
                .text("Lower")
                .update_while_editing(false),
        );
    }
}
