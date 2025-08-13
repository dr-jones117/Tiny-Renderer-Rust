use crate::algorithms::Algorithms;
use crate::graphics::{RenderOutputter, TinyRendererWindow, color};
use crate::renderer::TinyRenderer;

pub struct TinyRendererBuilder<T: RenderOutputter> {
    render_output: Option<T>,
    algorithms: Option<Algorithms<T>>,
    color: Option<color::RGBA>,
}

impl<T: RenderOutputter> TinyRendererBuilder<T> {
    pub fn new() -> TinyRendererBuilder<T> {
        TinyRendererBuilder {
            render_output: None,
            algorithms: None,
            color: None,
        }
    }

    pub fn with_render_output(mut self, render_output: T) -> TinyRendererBuilder<T> {
        self.render_output = Some(render_output);
        self
    }

    pub fn with_algorithms(mut self, algorithms: Algorithms<T>) -> TinyRendererBuilder<T> {
        self.algorithms = Some(algorithms);
        self
    }

    pub fn with_color(mut self, color: color::RGBA) -> TinyRendererBuilder<T> {
        self.color = Some(color);
        self
    }

    pub fn build(self) -> TinyRenderer<T> {
        let render_output = self.render_output.unwrap_or_else(|| {
            panic!("Cannot create a renderer with no output renderer.");
        });
        let algorithms = self.algorithms.unwrap_or_else(|| {
            panic!("Cannot create a renderer with no algorithms.");
        });
        let color = match self.color {
            Some(t) => t,
            None => color::RGBA {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
        };

        TinyRenderer::new(render_output, algorithms, color)
    }
}

impl TinyRendererBuilder<TinyRendererWindow> {
    pub fn with_target_fps(mut self, target_fps: usize) -> TinyRendererBuilder<TinyRendererWindow> {
        self.render_output
            .as_mut()
            .unwrap_or_else(|| panic!("Error setting the target fps."))
            .set_target_fps(target_fps);

        self
    }
}
