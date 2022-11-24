#[derive(Debug)]
pub struct Bounds {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    last_point: (f64, f64),
}

impl Bounds {
    pub fn new() -> Self {
        Bounds {
            x1: 0.0,
            y1: 0.0,
            x2: 0.0,
            y2: 0.0,
            last_point: (0.0, 0.0),
        }
    }

    pub fn move_last_point(&self, x: f64, y: f64, is_first_point: bool) -> Bounds {
        if is_first_point {
            return Bounds {
                x1: x,
                y1: y,
                x2: x,
                y2: y,
                last_point: (x, y),
            };
        }

        Bounds {
            x1: self.last_point.0,
            y1: self.last_point.1,
            x2: x,
            y2: y,
            last_point: (x, y),
        }
    }

    pub fn close(&self) -> Self {
        Bounds {
            x1: self.x1,
            y1: self.y1,
            x2: self.x2,
            y2: self.y2,
            last_point: (self.x1, self.y1),
        }
    }

    pub fn extends(&self, x: f64, y: f64) -> Self {
        Bounds {
            x1: if x < self.x1 { x } else { self.x1 },
            y1: if y < self.y1 { y } else { self.y1 },
            x2: if x > self.x2 { x } else { self.x2 },
            y2: if y > self.y2 { y } else { self.y2 },
            last_point: (x, y),
        }
    }

    pub fn x1(&self) -> f64 {
        self.x1
    }

    pub fn y1(&self) -> f64 {
        self.y1
    }

    pub fn x2(&self) -> f64 {
        self.x2
    }

    pub fn y2(&self) -> f64 {
        self.y2
    }

    pub fn last_point(&self) -> (f64, f64) {
        self.last_point
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_first_last_point() {
        let path = Bounds {
            x1: 0.0,
            y1: 0.0,
            x2: 0.0,
            y2: 0.0,
            last_point: (0.0, 0.0),
        };

        let path = path.move_last_point(10.0, 10.0, true);
        assert_eq!(path.x1, 10.0);
        assert_eq!(path.y1, 10.0);
        assert_eq!(path.x2, 10.0);
        assert_eq!(path.y2, 10.0);
        assert_eq!(path.last_point, (10.0, 10.0));

        let path = path.move_last_point(20.0, 20.0, false);
        assert_eq!(path.x1, 10.0);
        assert_eq!(path.y1, 10.0);
        assert_eq!(path.x2, 20.0);
        assert_eq!(path.y2, 20.0);
        assert_eq!(path.last_point, (20.0, 20.0));
    }

    #[test]
    fn close_path() {
        let path = Bounds {
            x1: 0.0,
            y1: 0.0,
            x2: 0.0,
            y2: 0.0,
            last_point: (0.0, 0.0),
        };

        let path = path.move_last_point(10.0, 10.0, false);
        let path = path.close();
        assert_eq!(path.x1, 0.0);
        assert_eq!(path.y1, 0.0);
        assert_eq!(path.x2, 10.0);
        assert_eq!(path.y2, 10.0);
        assert_eq!(path.last_point, (0.0, 0.0));
    }

    #[test]
    fn extends_path() {
        let path = Bounds {
            x1: 0.0,
            y1: 0.0,
            x2: 0.0,
            y2: 0.0,
            last_point: (0.0, 0.0),
        };

        let path = path.extends(10.0, 10.0);
        assert_eq!(path.x1, 0.0);
        assert_eq!(path.y1, 0.0);
        assert_eq!(path.x2, 10.0);
        assert_eq!(path.y2, 10.0);
        assert_eq!(path.last_point, (10.0, 10.0));
    }
}