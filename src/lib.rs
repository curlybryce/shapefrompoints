use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone, Copy, Hash, Eq, PartialOrd, Ord)]
pub struct Point(isize, isize);
impl Point {
    pub fn distance(&self, p: &Point) -> f32 {
        ((self.0 - p.0) as f32).hypot((self.1 - p.1) as f32)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct Float(isize, usize);
impl Float {
    pub fn from(f: f32) -> Float {
        Float(f as isize, (f.fract() * 1000000.0) as usize)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Connection(Option<Point>, Option<Point>);

pub struct Shape {
    map: HashMap<Point, Connection>,
    list: Vec<Point>,
    points: Vec<Point>,
}
impl Shape {
    pub fn new(map: HashMap<Point, Connection>, list: Vec<Point>, points: Vec<Point>) -> Shape {
        Shape {
            map: map,
            list: list,
            points: points,
        }
    }

    fn calc(&mut self) -> Option<Vec<Point>> {
        self.light_algo(
            None,
            self.map.clone(),
            self.list.clone(),
            self.points.clone(),
        )
    }

    fn light_algo(
        &mut self,
        prev: Option<&Point>,
        mut map: HashMap<Point, Connection>,
        mut list: Vec<Point>,
        mut points: Vec<Point>,
    ) -> Option<Vec<Point>> {
        // Initialize p.  Otherwise set p to current value
        let p;
        if list.len() == 0 {
            match points.pop() {
                Some(n) => {
                    list.push(n);
                    points.push(n);
                    p = n
                }
                None => return None,
            };
        } else {
            match list.pop() {
                Some(n) => {
                    list.push(n);
                    p = n
                }
                None => return None,
            }
        };

        // Create and sort a vec of (f32, Point)
        // Closest to farthest point
        let mut vec: Vec<(Float, Point)> = vec![];

        for point in &points {
            let d = Float::from(p.distance(&point));
            if point != &p {
                if prev == None {
                    vec.push((d, *point))
                } else if point != prev.unwrap() {
                    vec.push((d, *point))
                }
            }
        }
        vec.sort();
        vec.reverse();

        // Get the closest point
        let key = match vec.get(vec.len() - 1) {
            Some(n) => n,
            None => return None,
        };

        // For every point in the vec; if this distance
        // is equal to the closest
        // then iterate over it
        for point in &vec {
            if point.0 == key.0 {
                // Win Condition
                // If the second connection of the current point
                // is the closest point then return the final list
                // removing the last point
                let var = match map.get(&p) {
                    Some(n) => Some(n),
                    None => None,
                };
                match var {
                    Some(n) => {
                        if n.0.unwrap() == point.1 && list.len() - 1 == points.len() {
                            list.pop();
                            return Some(list.clone());
                        } else if list.len() - 1 == points.len() {
                            return None;
                        }
                    }
                    None => (),
                }

                // If the connections are full then return None
                match map.get(&point.1) {
                    Some(n) => {
                        if n.1 != None && n.0 != None {
                            return None;
                        }
                    }
                    None => (),
                }

                // If the current point has one connection then
                // add the closest distance point to its connections
                // otherwise initialize
                match map.get(&p) {
                    Some(n) => {
                        let v = match n.0 {
                            None => Connection(Some(point.1), None),
                            Some(_) => Connection(n.0, Some(point.1)),
                        };
                        map.insert(p, v);
                    }
                    None => {
                        map.insert(p, Connection(Some(point.1), None));
                    }
                }
                match map.get(&point.1) {
                    Some(n) => {
                        let v = match n.0 {
                            None => Connection(Some(p), None),
                            Some(_) => Connection(n.0, Some(p)),
                        };
                        map.insert(point.1, v);
                    }
                    None => {
                        map.insert(point.1, Connection(Some(p), None));
                    }
                }

                // Add the closest point to the final list
                // Remove the closest point from the remaining points
                // Iterate
                list.push(point.1);
                match self.light_algo(Some(&p), map.clone(), list.clone(), points.clone()) {
                    Some(n) => return Some(n),
                    None => return None,
                }
            }
        }

        // Default to returning nothing
        return None;
    }

    // fn heavy_algo(&self) -> Option<Vec<Point>> {}
}

// fn run() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_distance() {
        let p = Point(1, 1);
        let p2 = Point(5, 5);
        assert_eq!(p.distance(&p2), 5.656854);

        let p = Point(4, 1);
        let p2 = Point(5, 10);
        assert_eq!(p.distance(&p2), 9.055386);

        let p = Point(5, 3);
        let p2 = Point(5, 3);
        assert_eq!(p.distance(&p2), 0.0);
    }

    #[test]
    fn equal_distance() {
        let p = Point(3, 0);
        let p2 = Point(0, 1);
        assert_eq!(p.distance(&p2), 3.1622777);

        let p = Point(3, 0);
        let p2 = Point(0, -1);
        assert_eq!(p.distance(&p2), 3.1622777);
    }

    #[test]
    fn negative_distance() {
        let p = Point(1, -1);
        let p2 = Point(-5, 5);
        assert_eq!(p.distance(&p2), 8.485281);

        let p = Point(-4, -1);
        let p2 = Point(-5, -10);
        assert_eq!(p.distance(&p2), 9.055386);

        let p = Point(-5, -3);
        let p2 = Point(-5, -3);
        assert_eq!(p.distance(&p2), 0.0);
    }

    #[test]
    #[should_panic]
    fn light_algo_diamond() {
        let v = vec![Point(5, 0), Point(0, 2), Point(0, -2), Point(-5, 0)];
        let mut shape = Shape::new(HashMap::new(), vec![], v);

        match shape.calc() {
            Some(n) => n,
            None => panic!("Nothing returned"),
        };
    }

    #[test]
    fn light_algo_equal_distance() {
        let v = vec![Point(0, -2), Point(0, 2), Point(1, -3), Point(-3, 0)];
        let mut shape = Shape::new(HashMap::new(), vec![], v);

        let shape = match shape.calc() {
            Some(n) => n,
            None => panic!("Nothing returned"),
        };

        assert_eq!(
            vec![Point(-3, 0), Point(0, 2), Point(0, -2), Point(1, -3),],
            shape
        );
    }

    #[test]
    #[should_panic]
    fn light_algo_consume_all_points() {
        let v = vec![Point(-7, 0), Point(0, 2), Point(1, 1), Point(3, 0)];
        let mut shape = Shape::new(HashMap::new(), vec![], v);

        match shape.calc() {
            Some(n) => n,
            None => panic!("Nothing returned"),
        };
    }
    #[test]
    fn light_algo_one_way() {
        let v = vec![
            Point(1, -3),
            Point(1, 0),
            Point(6, -2),
            Point(5, 0),
            Point(3, -5),
            Point(0, -2),
            Point(6, -5),
            Point(3, 0),
            Point(0, 0),
        ];
        let mut shape = Shape::new(HashMap::new(), vec![], v);

        let shape = match shape.calc() {
            Some(n) => n,
            None => panic!("Nothing returned"),
        };

        assert_eq!(
            vec![
                Point(0, 0),
                Point(1, 0),
                Point(3, 0),
                Point(5, 0),
                Point(6, -2),
                Point(6, -5),
                Point(3, -5),
                Point(1, -3),
                Point(0, -2),
            ],
            shape
        )
    }
}
