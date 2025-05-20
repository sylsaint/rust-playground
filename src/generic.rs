struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
pub fn generic_run() {
    fn echo<T>(v: T) -> T {
        v
    }

    enum Either<E, T> {
        Left(E),
        Right(T),
    }

    let point = Point { x: 32, y: 32 };
    println!("point.x is {}", point.x);
    let pointf32: Point<f32> = Point { x: 32.1, y: 32.1 };
    println!("distance from origin is {}", pointf32.distance_from_origin());
}
