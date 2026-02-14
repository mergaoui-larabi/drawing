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

// impl Displayable for Point {
//     fn display(&mut self, _x: i32, _y: i32, _color: Color) {}
// }

// ============================
//  RECTANGLE
// ============================

#[derive(Debug)]
pub struct Rectangle {
    pub point1: Point,
    pub point2: Point,
}

impl Rectangle {
    pub fn new(point1: &Point, point2: &Point) -> Rectangle {
        Rectangle {
            point1: *point1,
            point2: *point2,
        }
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
        let x_min = self.point1.x.min(self.point2.x);
        let x_max = self.point1.x.max(self.point2.x);
        let y_min = self.point1.y.min(self.point2.y);
        let y_max = self.point1.y.max(self.point2.y);

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
        draw_line(img, &self.a, &self.b, color.clone());
        draw_line(img, &self.b, &self.c, color.clone());
        draw_line(img, &self.c, &self.a, color);
    }
}
fn draw_line(img: &mut Image, a: &Point, b: &Point, color: Color) {
    let mut x0 = a.x;
    let mut y0 = a.y;
    let x1 = b.x;
    let y1 = b.y;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    // let max_iterations = (dx + dy) * 2; // Limite le nombre d'itérations pour éviter les boucles infinies
    // let mut iterations = 0;

    if dx > dy {
        let mut err = dx / 2;
        while x0 != x1
        /*&& iterations < max_iterations*/
        {
            if x0 >= 0 && x0 < img.width && y0 >= 0 && y0 < img.height {
                img.set_pixel(x0, y0, color.clone()).unwrap();
            }
            err -= dy;
            if err < 0 {
                y0 += sy;
                err += dx;
            }
            x0 += sx;
            // iterations += 1;
        }
    } else {
        let mut err = dy / 2;
        while y0 != y1
        /*&& iterations < max_iterations*/
        {
            if x0 >= 0 && x0 < img.width && y0 >= 0 && y0 < img.height {
                img.set_pixel(x0, y0, color.clone()).unwrap();
            }
            err -= dx;
            if err < 0 {
                x0 += sx;
                err += dy;
            }
            y0 += sy;
            // iterations += 1;
        }
    }

    // Dessiner le dernier point
    if x0 >= 0 && x0 < img.width && y0 >= 0 && y0 < img.height {
        img.display(x0, y0, color);
    }
}
// ============================
// LINE
// ============================

pub struct Line {
    p1: Point,
    p2: Point,
}
impl Line {
    // pub fn new(p1: &Point, p2: &Point) -> Self {
    //     Line { p1: *p1, p2: *p2 }
    // }

    pub fn random(width: i32, height: i32) -> Self {
        Line {
            p1: Point::random(width, height),
            p2: Point::random(width, height),
        }
    }
}
impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let color = self.color();
        let dx = self.p2.x - self.p1.x;
        let dy = self.p2.y - self.p1.y;

        let steps = dx.abs().max(dy.abs());

        for i in 0..=steps {
            let x = self.p1.x + i * dx / steps;
            let y = self.p1.y + i * dy / steps;
            image.display(x, y, color.clone());
        }
    }

    fn color(&self) -> Color {
        let n1 = rand::rng().random_range(0..255);
        let n2 = rand::rng().random_range(0..255);
        let n3 = rand::rng().random_range(0..255);
        Color::rgb(n1, n2, n3)
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
            center: Point::new(center.x, center.y),
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
            // Angle en radians
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
