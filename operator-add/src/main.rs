#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> std::ops::Add for Point<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    // println!("{:?}, {:?}", p1, p2);
    println!("{:?}", p3);
}
