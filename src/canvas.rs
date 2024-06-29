use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct Canvas {
    pub canvas: HtmlCanvasElement,
    pub ctx: CanvasRenderingContext2d,
    scaled_width: u32,
    scaled_height: u32,
    width: u32,
    height: u32,
}

impl Canvas {
    pub fn new(attr_id: &str, width: u32, height: u32) -> Option<Canvas> {
        let document = web_sys::window()?.document()?;
        let canvas_element = document
            .query_selector(attr_id).ok()??
            .dyn_into::<HtmlCanvasElement>().ok()?;

        let ctx: CanvasRenderingContext2d = canvas_element
            .get_context("2d").ok()?? // Note: Changed to "2d"
            .dyn_into::<CanvasRenderingContext2d>().ok()?;

        let scaled_width = canvas_element.width() / width;
        let scaled_height = canvas_element.height() / height;

        Some(Canvas {
            canvas: canvas_element,
            ctx,
            scaled_width,
            scaled_height,
            width,
            height,
        })
    }

    pub fn draw(&self, x: u32, y: u32, color: &str) {
        assert!(x < self.width);
        assert!(y < self.height);

        self.ctx.set_fill_style(&color.into());

        let x = x * self.scaled_width;
        let y = y * self.scaled_height;

        self.ctx.fill_rect(
            f64::from(x) as f64,
            f64::from(y) as f64,
            f64::from(self.scaled_width) as f64,
            f64::from(self.scaled_height) as f64,
        );
    }

    pub fn clear_all(&self) {
        self.ctx.set_fill_style(&"white".into());
        self.ctx.fill_rect(
            0.0,
            0.0,
            f64::from(self.width * self.scaled_width) as f64,
            f64::from(self.height * self.scaled_height) as f64,
        );
    }

    pub fn draw_border(&self) {
        self.ctx.set_stroke_style(&JsValue::from_str("black"));
        self.ctx.begin_path();
        self.ctx.rect(
            0.0,
            0.0,
            f64::from(self.canvas.width()),
            f64::from(self.canvas.height()),
        );
        self.ctx.stroke();
    }

    pub fn display_scores(&self, score: u32, high_score: u32) {
        let document = web_sys::window().unwrap().document().unwrap();

        let score_element = document.get_element_by_id("score")
            .unwrap_or_else(|| {
                let elem = document.create_element("div").unwrap();
                elem.set_id("score");
                document.body().unwrap().append_child(&elem).unwrap();
                elem
            });
        score_element.set_inner_html(&format!("Score: {}", score));

        let high_score_element = document.get_element_by_id("high_score")
            .unwrap_or_else(|| {
                let elem = document.create_element("div").unwrap();
                elem.set_id("high_score");
                document.body().unwrap().append_child(&elem).unwrap();
                elem
            });
        high_score_element.set_inner_html(&format!("High Score: {}", high_score));
    }
}
