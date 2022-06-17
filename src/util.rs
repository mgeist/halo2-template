use plotters::{coord::Shift, prelude::*};

pub struct DrawConfig<'a> {
    path: &'a str,
    resolution: (u32, u32),
    title: &'a str,
    title_font: &'a str,
    title_size: u32,
}

impl<'a> DrawConfig<'a> {
    pub fn new(path: &str) -> DrawConfig {
        DrawConfig {
            resolution: (800, 600),
            path,
            title: "",
            title_font: "sans-serif",
            title_size: 60,
        }
    }

    pub fn with_title(mut self, title: &'a str) -> DrawConfig {
        self.title = title;
        self
    }

    pub fn with_resolution(mut self, resolution: (u32, u32)) -> DrawConfig<'a> {
        self.resolution = resolution;
        self
    }

    pub fn build(&self) -> DrawingArea<BitMapBackend, Shift> {
        let area = BitMapBackend::new(self.path, self.resolution).into_drawing_area();
        area.fill(&WHITE).unwrap();

        area.titled(self.title, (self.title_font, self.title_size))
            .unwrap()
    }
}
