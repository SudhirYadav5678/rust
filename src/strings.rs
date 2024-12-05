fn strign(s: &str) -> usize {
    s.chars().count()
}

fn main() {
    let strings = String::from("Hello");
    let ans = strign(&strings);
    println!("{}", ans)
}
