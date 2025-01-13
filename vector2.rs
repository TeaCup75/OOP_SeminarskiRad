struct Vector2 {
    x: f64,
    y: f64,
}

impl Vector2 {
    fn new(x: f64, y: f64) -> Self {
        Vector2 { x, y }
    }

    fn add(&self, other:&Vector2) -> Self {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(&self, other:&Vector2) -> Self {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    fn normalize(&self) -> Self {
        let magnitude = self.magnitude();

        Vector2 {
            x: self.x / magnitude,
            y: self.y / magnitude,
        }
    }

    fn print(&self) {
        println!("({}, {})", self.x, self.y);
    }

    fn clear(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }
}

fn main() {
    let a = Vector2::new(1.0, 2.0);
    let b = Vector2::new(3.0, 4.0);

    let c = a.add(&b);
    c.print();
    println!("magnitude = {}", c.magnitude());

    let mut nc = c.normalize();
    nc.print();
    println!("magnitude = {}", nc.magnitude());

    nc.clear();
    nc.print();
    println!("magnitude = {}", nc.magnitude());
}
