use egui::{Color32, Ui};
use egui::plot::{Line, LineStyle, Plot, Values};

#[derive(PartialEq)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Canvas {
    data: Vec<Vec<char>>,
    time: f64,
}

impl Canvas {
    pub fn new() -> Canvas {
        return Canvas {
            data: Vec::new(),
            time: 0.0,
        };
    }
}

impl Canvas {
    fn sin(&self) -> Line {
        let line_style: LineStyle = LineStyle::Solid;
        let time = self.time;
        Line::new(Values::from_explicit_callback(
            move |x| 0.5 * (2.0 * x).sin() * time.sin(),
            ..,
            512,
        ))
            .color(Color32::from_rgb(200, 100, 100))
            .style(line_style)
            .name("wave")
    }

    pub(crate) fn ui(&mut self, ui: &mut Ui) {
        let mut plot = Plot::new("canvas");
        plot.show(ui, |plot_ui| {
            plot_ui.line(self.sin())
        });
    }
}
