// Meter

use egui::Ui;

pub struct Bars {
    bars: u8,
}

impl Default for Bars {
    fn default() -> Self {
        Self { bars: 4 }
    }
}

impl Bars {
    pub fn update(&mut self, ui: &mut Ui) {
        ui.label(format!("Bars: {}", self.bars));

        // Bars
        ui.add(
            egui::Slider::new(&mut self.bars, 1..=32)
                .text("bars")
                .update_while_editing(false),
        );
    }
}
