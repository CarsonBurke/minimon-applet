use std::collections::VecDeque;

use cosmic::{Element, iced::Length};
use plotters::{
    series::AreaSeries,
    style::{RGBAColor, ShapeStyle},
};
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

use crate::{app::Message, config::GraphColors};

#[derive(Debug)]
pub struct TimeLineGraph {
    pub max_samples: i32,
    pub data: VecDeque<i32>,
    pub colors: GraphColors,
}
impl Chart<Message> for TimeLineGraph {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, state: &Self::State, mut builder: ChartBuilder<DB>) {
        let mut chart = builder
            .build_cartesian_2d(0..self.max_samples, 0..100)
            .expect("Failed to build chart");

        let mut mesh = chart.configure_mesh();
        mesh.bold_line_style(
            ShapeStyle::from(RGBAColor {
                0: self.colors.color2.red,
                1: self.colors.color2.green,
                2: self.colors.color2.blue,
                3: self.colors.color2.alpha as f64 / 255.,
            })
            /* .stroke_width(1), */
        )
        /* .axis_style(
            ShapeStyle::from(RGBAColor {
                0: self.colors.color1.red,
                1: self.colors.color1.green,
                2: self.colors.color1.blue,
                3: self.colors.color1.alpha as f64 / 255.,
            })
            /* .stroke_width(1), */
        ) */
        .max_light_lines(2).y_labels(2).x_labels(2).disable_axes();

        mesh.draw().expect("Failed to draw mesh");

        chart
            .draw_series(
                AreaSeries::new(
                    self.data.iter().enumerate().map(|(x, y)| (x as i32, *y)),
                    0,
                    RGBAColor {
                        0: self.colors.color4.red,
                        1: self.colors.color4.green,
                        2: self.colors.color4.blue,
                        3: self.colors.color4.alpha as f64 / 255. * 0.5, // reduced transparency to make the area more transparent tha the line
                    },
                )
                .border_style(
                    ShapeStyle::from(RGBAColor {
                        0: self.colors.color4.red,
                        1: self.colors.color4.green,
                        2: self.colors.color4.blue,
                        3: self.colors.color4.alpha as f64 / 255.,
                    })
                    .stroke_width(1),
                ),
            )
            .expect("Failed to draw chart");
    }
}

impl TimeLineGraph {
    pub fn new() -> Self {
        Self {
            max_samples: 100,
            data: VecDeque::new(),
            colors: GraphColors::default(),
        }
    }

    pub fn view(&self, width: f32, height: f32) -> Element<Message> {
        ChartWidget::new(self)
            .width(Length::Fixed(width))
            .height(Length::Fixed(height))
            .into()
    }
}

#[derive(Debug)]
struct BarGraph;
impl Chart<Message> for BarGraph {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, state: &Self::State, mut builder: ChartBuilder<DB>) {}
}

impl BarGraph {
    pub fn view(&self, width: f32, height: f32) -> Element<Message> {
        ChartWidget::new(self)
            .width(Length::Fixed(width))
            .height(Length::Fixed(height))
            .into()
    }
}
