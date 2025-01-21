enum Shape {
    Circle { radius: f64 },
    Square { border: f64 },
    Rectangle { width: f64, height: f64 },
}

impl Shape {
    // TODO: Implement the `radius` method using
    //  either an `if let` or a `let/else`.
    pub fn radius(&self) -> f64 {
        if let Shape::Circle { radius } = self {
            return *radius;
        } else {
            panic!("Not a circle");
        }

        if let Shape::Square { border } = self {
            return *border;
        } else {
            panic!("Not a square");
        }

        if let Shape::Rectangle { width, height } = self {
            return width * height;
        } else {
            panic!("Not a rectangle");
        }

        return -1.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let _ = Shape::Circle { radius: 1.0 }.radius();
    }

    #[test]
    #[should_panic]
    fn test_square() {
        let _ = Shape::Square { border: 1.0 }.radius();
    }

    #[test]
    #[should_panic]
    fn test_rectangle() {
        let _ = Shape::Rectangle {
            width: 1.0,
            height: 2.0,
        }
        .radius();
    }
}
