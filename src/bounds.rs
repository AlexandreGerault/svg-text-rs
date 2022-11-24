use std::cmp::{min, max};

#[derive(Debug)]
struct Bounds<T: std::cmp::Ord + Copy> {
    x1: T,
    y1: T,
    x2: T,
    y2: T,
    last_point: (T, T),
}

impl<T: std::cmp::Ord + Copy> Bounds<T> {
    fn move_last_point(&self, x: T, y: T, is_first_point: bool) -> Bounds<T> {
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

    fn close(&self) -> Self {
        Bounds {
            x1: self.x1,
            y1: self.y1,
            x2: self.x2,
            y2: self.y2,
            last_point: (self.x1, self.y1),
        }
    }

    fn extends(&self, x: T, y: T) -> Self {
        Bounds {
            x1: min(self.x1, x),
            y1: min(self.y1, y),
            x2: max(self.x2, x),
            y2: max(self.y2, y),
            last_point: (x, y),
        }
    }
}

#[test]
fn move_first_last_point() {
    let path = Bounds {
        x1: 0,
        y1: 0,
        x2: 0,
        y2: 0,
        last_point: (0, 0),
    };

    let path = path.move_last_point(10, 10, true);
    assert_eq!(path.x1, 10);
    assert_eq!(path.y1, 10);
    assert_eq!(path.x2, 10);
    assert_eq!(path.y2, 10);
    assert_eq!(path.last_point, (10, 10));

    let path = path.move_last_point(20, 20, false);
    assert_eq!(path.x1, 10);
    assert_eq!(path.y1, 10);
    assert_eq!(path.x2, 20);
    assert_eq!(path.y2, 20);
    assert_eq!(path.last_point, (20, 20));
}

#[test]
fn close_path() {
    let path = Bounds {
        x1: 0,
        y1: 0,
        x2: 0,
        y2: 0,
        last_point: (0, 0),
    };

    let path = path.move_last_point(10, 10, false);
    let path = path.close();
    assert_eq!(path.x1, 0);
    assert_eq!(path.y1, 0);
    assert_eq!(path.x2, 10);
    assert_eq!(path.y2, 10);
    assert_eq!(path.last_point, (0, 0));
}

#[test]
fn extends_path() {
    let path = Bounds {
        x1: 0,
        y1: 0,
        x2: 0,
        y2: 0,
        last_point: (0, 0),
    };

    let path = path.extends(10, 10);
    assert_eq!(path.x1, 0);
    assert_eq!(path.y1, 0);
    assert_eq!(path.x2, 10);
    assert_eq!(path.y2, 10);
    assert_eq!(path.last_point, (10, 10));
}