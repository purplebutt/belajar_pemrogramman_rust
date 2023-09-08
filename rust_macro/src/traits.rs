trait Shape {
    fn area(&self) -> u32;
}
trait Drawable {
    fn draw(&self);
}

trait Circle: Shape + Drawable {
    fn radius(&self) -> f32;
}
