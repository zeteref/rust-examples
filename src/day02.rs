// Day 2: Drawable Trait and total_area
//
// Define shapes that implement a shared `Drawable` trait and write a function
// to sum the area of their bounding boxes.
//
// Learning goals:
//   - Defining and implementing traits
//   - Dynamic dispatch with `Box<dyn Trait>`
//   - Working with f64 formatting
//   - Understanding geometric vs bounding-box area

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
    fn bounding_box(&self) -> (f64, f64, f64, f64); // (x, y, width, height)
}

// Implement Drawable for Circle
impl Drawable for Circle {
    fn draw(&self) -> String {
        todo!("Implement draw for Circle")
    }

    fn bounding_box(&self) -> (f64, f64, f64, f64) {
        todo!("Implement bounding_box for Circle")
    }
}

// Implement Drawable for Rectangle
impl Drawable for Rectangle {
    fn draw(&self) -> String {
        todo!("Implement draw for Rectangle")
    }

    fn bounding_box(&self) -> (f64, f64, f64, f64) {
        todo!("Implement bounding_box for Rectangle")
    }
}

/// Returns the sum of bounding box areas (width * height) for all shapes.
/// The area of an empty slice is 0.0.
pub fn total_area(shapes: &[Box<dyn Drawable>]) -> f64 {
    todo!("Implement total_area")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_circle(cx: f64, cy: f64, radius: f64) -> Box<dyn Drawable> {
        Box::new(Circle { cx, cy, radius })
    }

    fn make_rect(x: f64, y: f64, width: f64, height: f64) -> Box<dyn Drawable> {
        Box::new(Rectangle { x, y, width, height })
    }

    #[test]
    fn circle_draw_output() {
        let c = Circle { cx: 1.0, cy: 2.0, radius: 3.0 };
        assert_eq!(c.draw(), "circle at (1.0,2.0) r=3.0");
    }

    #[test]
    fn rectangle_draw_output() {
        let r = Rectangle { x: 0.0, y: 0.0, width: 4.0, height: 5.0 };
        assert_eq!(r.draw(), "rect at (0.0,0.0) 4.0x5.0");
    }

    #[test]
    fn circle_bounding_box() {
        let c = Circle { cx: 1.0, cy: 2.0, radius: 3.0 };
        let (x, y, w, h) = c.bounding_box();
        assert_eq!((x, y, w, h), (-2.0, -1.0, 6.0, 6.0));
    }

    #[test]
    fn rectangle_bounding_box() {
        let r = Rectangle { x: 0.0, y: 0.0, width: 4.0, height: 5.0 };
        let (x, y, w, h) = r.bounding_box();
        assert_eq!((x, y, w, h), (0.0, 0.0, 4.0, 5.0));
    }

    #[test]
    fn total_area_empty_slice_is_zero() {
        let shapes: Vec<Box<dyn Drawable>> = vec![];
        assert_eq!(total_area(&shapes), 0.0);
    }

    #[test]
    fn total_area_single_circle() {
        // Circle r=3.0: bounding box = 2r × 2r = 6 × 6 = 36
        let shapes: Vec<Box<dyn Drawable>> = vec![make_circle(1.0, 2.0, 3.0)];
        assert_eq!(total_area(&shapes), 36.0);
    }

    #[test]
    fn total_area_single_rectangle() {
        let shapes: Vec<Box<dyn Drawable>> = vec![make_rect(0.0, 0.0, 4.0, 5.0)];
        assert_eq!(total_area(&shapes), 20.0);
    }

    #[test]
    fn total_area_sum_bounding_box_not_geometric_area() {
        // Circle r=1.0: bounding box 2×2 = 4, NOT πr² ≈ 3.14
        let shapes: Vec<Box<dyn Drawable>> = vec![make_circle(0.0, 0.0, 1.0)];
        assert_eq!(total_area(&shapes), 4.0);
    }

    #[test]
    fn total_area_mixed_shapes() {
        // Circle r=2: bounding box 4×4 = 16
        // Rectangle 3×5 = 15
        // Total = 31
        let shapes: Vec<Box<dyn Drawable>> = vec![
            make_circle(0.0, 0.0, 2.0),
            make_rect(0.0, 0.0, 3.0, 5.0),
        ];
        assert_eq!(total_area(&shapes), 31.0);
    }

    #[test]
    fn draw_format_uses_one_decimal_place() {
        // Values like 5.0 should format as "5.0", not "5" or "5.00"
        let r = Rectangle { x: 0.0, y: 0.0, width: 5.0, height: 5.0 };
        let s = r.draw();
        assert!(s.contains("5.0"), "Expected '5.0' in output, got: {}", s);
    }
}
