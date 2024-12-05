struct Rect {
    w: i32,
    h: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        self.w * self.h
    }
    fn perimeter(&self) -> i32 {
        2 * (self.w + self.h)
    }
}

fn main() {
    let are = Rect { w: 3, h: 5 };
    println!("{}", are.area());
    println!("{}", are.perimeter());
}
