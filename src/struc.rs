struct User {
    name: String,
    last_name: String,
    no: u32,
    age: u32,
    active: bool,
}

impl User {
    fn print_name(&self) -> String {
        println!("{}", name + last_name);
    }
}

fn main() {
    let user1 = User {
        name: String::from("Sudhir"),
        last_name: String::from("Yadav"),
        no: 124556,
        age: 22,
        active: true,
    };
    println!("{}", user1.name);
}

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
