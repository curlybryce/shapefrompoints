use shapefrompoints::Point;

fn main() {
    let v = vec![Point(-7, 0), Point(0, 2), Point(1, 1), Point(3, 0)];
    shapefrompoints::run(v);
}