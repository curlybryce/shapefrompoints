use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone, Copy, Hash, Eq, PartialOrd, Ord)]
pub struct Point(isize, isize);
impl Point {
    pub fn distance(&self, p: &Point) -> f32 {
        ((self.0 - p.0) as f32).hypot((self.1 - p.1) as f32)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Float(isize, usize);
impl Float {
    pub fn from(f: f32) -> Float {
        Float(
            f as isize,
            (f.fract() * 1000000.0) as usize,
        )
    }
}

#[derive(Debug)]
struct Connection(Option<Point>, Option<Point>);

pub struct Shape {
    points: Vec<Point>,
}
impl Shape {
    pub fn new(vec: Vec<Point>) -> Shape {
        Shape{
            points: vec,
        }
    }

    // fn calc() -> Option<Vec<Point>> {}

    fn light_algo(
        &mut self,
        prev: Option<&Point>,
        mut map: HashMap<Point, Connection>,
        mut list: Vec<Point>
    ) -> Option<Vec<Point>> {
        // Initialize p.  Otherwise set p to current value
        let p;
        if list.len() == 0 {
            match self.points.pop() {
                Some(n) => {list.push(n); self.points.push(n); p = n},
                None => return None
            };
        } else {
            match list.pop() {
                Some(n) => {
                    list.push(n);
                    p = n
                },
                None => return None
            }
        };

        // Create and sort a vec of (f32, Point)
        // Closest to farthest point
        let mut vec: Vec<(Float, Point)> = vec![];

        for point in &self.points {
            let d =  Float::from(p.distance(&point));
            if point != &p {
                if prev == None {
                    vec.push((d, *point))
                } else if point != prev.unwrap() {
                    println!("{:?}", point);
                    vec.push((d, *point))
                }
            }
        }
        vec.sort();
        
        // Get the closest point and remove it from the vec
        // skip if it is the current point
        println!("{:?}", vec);
        let key = vec.get(0).unwrap().1;
        vec.remove(0);
        
        // Win Condition
        // If the second connection of the current point
        // is the closest point then return the final list
        // removing the last point
        println!("{:?}", p);
        let var = match map.get(&p) {
            Some(n) => n.0,
            None => None
        };
        match var {
            Some(n) => {
                if n == p {
                    list.pop();
                    return Some(list.clone());
                } else {
                    println!("{:?} != {:?}", n, p);
                    return None;
                }
            },
            None => ()
        }

        // If the current point has one connection then
        // add the closest distance point to its connections
        // otherwise initialize
        for point in vec![&p, &key] {
            let v = match map.get_mut(point) {
                None => Connection(Some(p), None),
                Some(n) => Connection(n.1, Some(key)),
            };
            map.insert(p, v);
        }

        // Add the closest point to the final list
        // Remove the closest point from the remaining points
        // Iterate
        list.push(key);
        println!("");
        return self.light_algo(Some(&p), map, list)

    }
    
    // fn heavy_algo(&self) -> Option<Vec<Point>> {}
}

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
    fn light_algo_diamond_fail() {
        let v = vec![
            Point(3, 0),
            Point(0, 2),
            Point(0, -2),
            Point(-3, 0)
        ];
        let mut shape = Shape::new(v);


        let shape = match shape.light_algo(None, HashMap::new(), vec![]) {
            Some(n) => n,
            None => panic!("Nothing returned")
        };

        assert_eq!(vec![
            Point(3, 0),
            Point(0, 2),
            Point(0, -2),
            Point(-3, 0),
            ],
            shape
        );
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
        let mut shape = Shape::new(v);


        let shape = match shape.light_algo(None, HashMap::new(), vec![]) {
            Some(n) => n,
            None => panic!("Nothing returned")
        };

        assert_eq!(vec![
            Point(0, 0),
            Point(1, 0),
            Point(3, 0),
            Point(5, 0),
            Point(6, -2),
            Point(6, -5),
            Point(3, -5),
            Point(1, -3),
            Point(0, -2),
        ], shape)
    }
}
