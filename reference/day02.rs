pub struct Circle {
    pub cx: f64,
    pub cy: f64,
    pub radius: f64,
}

pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub trait Drawable {
    fn draw(&self) -> String;
    fn bounding_box(&self) -> (f64, f64, f64, f64);
}

impl Drawable for Circle {
    fn draw(&self) -> String {
        format!("circle at ({:.1},{:.1}) r={:.1}", self.cx, self.cy, self.radius)
    }

    fn bounding_box(&self) -> (f64, f64, f64, f64) {
        (
            self.cx - self.radius,
            self.cy - self.radius,
            self.radius * 2.0,
            self.radius * 2.0,
        )
    }
}

impl Drawable for Rectangle {
    fn draw(&self) -> String {
        format!(
            "rect at ({:.1},{:.1}) {:.1}x{:.1}",
            self.x, self.y, self.width, self.height
        )
    }

    fn bounding_box(&self) -> (f64, f64, f64, f64) {
        (self.x, self.y, self.width, self.height)
    }
}

pub fn total_area(shapes: &[Box<dyn Drawable>]) -> f64 {
    shapes
        .iter()
        .map(|s| {
            let (_, _, w, h) = s.bounding_box();
            w * h
        })
        .sum()
}
