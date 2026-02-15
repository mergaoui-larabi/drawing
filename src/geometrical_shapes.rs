use rand::Rng;
use raster::{Color, Image};

pub trait Drawable {
    fn draw(&self, img: &mut Image);
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// ============================
//   POINT
// ============================

#[derive(Debug, Copy, Clone)]

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn random(width: i32, height: i32) -> Point {
        Point {
            x: rand::rng().random_range(0..width),
            y: rand::rng().random_range(0..height),
        }
    }
}

impl Drawable for Point {
    fn color(&self) -> Color {
        let n1 = rand::rng().random_range(0..255);
        let n2 = rand::rng().random_range(0..255);
        let n3 = rand::rng().random_range(0..255);
        Color::rgb(n1, n2, n3)
    }

    fn draw(&self, img: &mut Image) {
        img.display(self.x, self.y, self.color());
    }
}
// ============================
// LINE
// ============================

pub struct Line {
    p1: Point,
    p2: Point,
    color: Color,
}

impl Line {
    pub fn new(p1: Point, p2: Point, color: Color) -> Self {
        Line {
            p1: p1,
            p2: p2,
            color: color,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let n1 = rand::rng().random_range(0..255);
        let n2 = rand::rng().random_range(0..255);
        let n3 = rand::rng().random_range(0..255);
        let color = Color::rgb(n1, n2, n3);

        Line {
            p1: Point::random(width, height),
            p2: Point::random(width, height),
            color: color,
        }
    }
}

impl Drawable for Line {
    fn draw(&self, img: &mut Image) {
        let mut x1 = self.p1.x;
        let mut y1 = self.p1.y;
        let x2 = self.p2.x;
        let y2 = self.p2.y;

        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();

        let sx = if x2 < x1 { -1 } else { 1 };
        let sy = if y2 < y1 { -1 } else { 1 };

        let mut err = dx + dy;

        loop {
            img.display(x1, y1, self.color.clone());

            if x1 == x2 && y1 == y2 {
                break;
            }

            let e2 = 2 * err; // err_float >= 0.5 / 2 => err_int = err_float * dx or / dy => 2 *err >= dy or 2 err <= dx

            if e2 >= dy {
                err += dy;
                x1 += sx;
            }

            if e2 <= dx {
                err += dx;
                y1 += sy;
            }
        }
    }

    fn color(&self) -> Color {
        let n1 = rand::rng().random_range(0..255);
        let n2 = rand::rng().random_range(0..255);
        let n3 = rand::rng().random_range(0..255);
        Color::rgb(n1, n2, n3)
    }
}
// ============================
//  RECTANGLE
// ============================

#[derive(Debug)]
pub struct Rectangle {
    pub p1: Point,
    pub p2: Point,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Rectangle {
        Rectangle { p1: *p1, p2: *p2 }
    }
}

impl Drawable for Rectangle {
    fn color(&self) -> Color {
        let n1 = rand::rng().random_range(0..255);
        let n2 = rand::rng().random_range(0..255);
        let n3 = rand::rng().random_range(0..255);
        Color::rgb(n1, n2, n3)
    }

    fn draw(&self, img: &mut Image) {
        let color = self.color();
        let x_min = self.p1.x.min(self.p2.x);
        let x_max = self.p1.x.max(self.p2.x);
        let y_min = self.p1.y.min(self.p2.y);
        let y_max = self.p1.y.max(self.p2.y);

        for x in x_min..=x_max {
            img.display(x, y_min, color.clone());
            img.display(x, y_max, color.clone());
        }

        for y in y_min..=y_max {
            img.display(x_min, y, color.clone());
            img.display(x_max, y, color.clone());
        }
    }
}

// ============================
// Triangle
// ============================
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Triangle {
        Triangle {
            a: *a,
            b: *b,
            c: *c,
        }
    }
}

impl Drawable for Triangle {
    fn color(&self) -> Color {
        let n1 = rand::rng().random_range(0..255);
        let n2 = rand::rng().random_range(0..255);
        let n3 = rand::rng().random_range(0..255);
        Color::rgb(n1, n2, n3)
    }
    fn draw(&self, img: &mut Image) {
        let color = self.color();
        Line::new(self.a, self.b, color.clone()).draw(img);
        Line::new(self.b, self.c, color.clone()).draw(img);
        Line::new(self.c, self.a, color.clone()).draw(img);
    }
}

// ===============================
//         CIRCLE
// ===============================

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Circle {
        Circle {
            center: *center,
            radius,
        }
    }

    pub fn random(width: i32, height: i32) -> Circle {
        Self::new(
            &Point::random(width, height),
            rand::rng().random_range(30..250),
        )
    }
}

impl Drawable for Circle {
    fn color(&self) -> Color {
        let n1 = rand::rng().random_range(0..255);
        let n2 = rand::rng().random_range(0..255);
        let n3 = rand::rng().random_range(0..255);
        Color::rgb(n1, n2, n3)
    }

    fn draw(&self, img: &mut Image) {
        let color = self.color();
        let mut i = 0.0;
        while i < 360.0 {
            // Angle in radians
            let angle = (i as f64).to_radians();

            // Coordonnées sur le cercle
            let x = self.center.x + (self.radius as f64 * angle.cos()).round() as i32;
            let y = self.center.y + (self.radius as f64 * angle.sin()).round() as i32;

            // Dessiner si dans les limites
            if x >= 0 && x < img.width && y >= 0 && y < img.height {
                img.set_pixel(x, y, color.clone()).unwrap();
            }
            i += 0.1; // Incrémenter l'angle
        }
    }
}

pub struct Pentagon {
    pub center: Point,
    pub radius: i32,
}

impl Pentagon {
    
    pub fn random(width: i32, height: i32) -> Self {
        Pentagon {
            center: Point::random(width, height),
            radius: rand::rng().random_range(30..250),
        }
    }
}

impl Drawable for Pentagon {
    fn color(&self) -> Color {
        let n1 = rand::rng().random_range(0..255);
        let n2 = rand::rng().random_range(0..255);
        let n3 = rand::rng().random_range(0..255);
        Color::rgb(n1, n2, n3)
    }

    fn draw(&self, img: &mut Image) {
        let color = self.color();
        let mut points = Vec::new();

        for i in 0..5 {
            let angle_deg = i as f64 * 72.0; // 5 points
            let angle_rad = angle_deg.to_radians(); // convert to radians
            let x = self.center.x as f64 + self.radius as f64 * angle_rad.cos();
            let y = self.center.y as f64 + self.radius as f64 * angle_rad.sin();
            points.push(Point::new(x.round() as i32, y.round() as i32));
        }

        for i in 0..points.len() {
            let p1 = &points[i];
            let p2 = &points[(i + 1) % points.len()]; // wrap around
            let line = Line::new(*p1, *p2, color.clone());
            line.draw(img);
        }
    }
}
