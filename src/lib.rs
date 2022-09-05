#[derive(PartialEq, Debug)]
pub struct Point(f32, f32);
impl Point {
    pub fn distance(&self, p: &Point) -> f32 {
        (self.0 - p.0).hypot(self.1 - p.1)
    }
}

pub struct Shape {
    vec: Vec<Point>
}
impl Shape {
    pub fn new(vec: Vec<Point>) -> Shape {
        Shape{
            vec: vec,
        }
    }

    // fn calc() -> Option<Vec<Point>> {}

    // Not actually implemented correctly ;3
    fn light_algo(&self, p: Point, remaining: Vec<Point>, mut current: Option<Vec<Point>>) -> Option<Vec<Point>> {
        let len = remaining.len();
        
        if &len == &0 {
            let mut c = current.unwrap();
            c.push(p);
            return Some(c)
        }
        
        let mut greatest_distance = None;
        let mut vec = vec![];
        let mut final_vec = vec![];
        let mut final_point: Option<Point> = None;
        
        for point in remaining {
            let d = p.distance(&point);
            
            vec.push((d, point));
            
            if greatest_distance == None {
                greatest_distance = Some(d);
            }

            if d < greatest_distance.unwrap() {
                greatest_distance = Some(d);
            }
        }
        
        for t in vec {
            if t.0 == greatest_distance.unwrap() {
                final_point = Some(t.1);
            } else {
                final_vec.push(t.1);
            }
        }
        
        match current {
            None => current = Some(vec![p]),
            Some(mut n) => current = {n.push(p); Some(n)},
        };

        match final_point {
            None => return None,
            Some(_) => ()
        }

        return self.light_algo(final_point.unwrap(), final_vec, current)
    }
    
    // fn heavy_algo(&self) -> Option<Vec<Point>> {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_distance() {
        let p = Point(1.0, 1.0);
        let p2 = Point(5.0, 5.0);
        assert_eq!(p.distance(&p2), 5.656854);

        let p = Point(4.0, 1.0);
        let p2 = Point(5.0, 10.0);
        assert_eq!(p.distance(&p2), 9.055386);

        let p = Point(5.0, 3.0);
        let p2 = Point(5.0, 3.0);
        assert_eq!(p.distance(&p2), 0.0);
    }

    #[test]
    fn negative_distance() {
        let p = Point(1.0, -1.0);
        let p2 = Point(-5.0, 5.0);
        assert_eq!(p.distance(&p2), 8.485281);

        let p = Point(-4.0, -1.0);
        let p2 = Point(-5.0, -10.0);
        assert_eq!(p.distance(&p2), 9.055386);

        let p = Point(-5.0, -3.0);
        let p2 = Point(-5.0, -3.0);
        assert_eq!(p.distance(&p2), 0.0);
    }

    #[test]
    fn light_algo_diamond() {
        let shape = Shape::new(vec![]);

        let v = vec![
            Point(1.0, 0.0),
            Point(-1.0, 0.0),
            Point(3.0, -1.0)
        ];

        let shape = match shape.light_algo(Point(-3.0, 1.0), v, None) {
            Some(n) => n,
            None => panic!()
        };

        assert_eq!(vec![
            Point(-3.0, 1.0),
            Point(1.0, 0.0),
            Point(3.0, -1.0),
            Point(-1.0, 0.0),
            ],
            shape
        );
    }

    #[test]
    fn light_algo_one_way() {
        let shape = Shape::new(vec![]);

        let v = vec![
            Point(1.0, -4.0),
            Point(1.0, 0.0),
            Point(6.0, -2.0),
            Point(6.0, 0.0),
            Point(3.0, -5.0),
            Point(6.0, -5.0),
            Point(3.0, 0.0),
        ];

        let shape = match shape.light_algo(Point(0.0, 0.0), v, None) {
            Some(n) => n,
            None => panic!()
        };

        assert_eq!(vec![
            Point(0.0, 0.0),
            Point(1.0, 0.0),
            Point(3.0, 0.0),
            Point(6.0, 0.0),
            Point(6.0, -2.0),
            Point(6.0, -5.0),
            Point(3.0, -5.0),
            Point(1.0, -4.0),
        ], shape)
    }
}
