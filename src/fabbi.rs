fn main() {
    println!("Hello");
    let ans = fabbi(12);
    println!(ans);
}

fn fabbi(num: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    if num == 0 {
        return 0;
    }
    if num == 1 {
        return 1;
    }

    for i in 1..num - 2 {
        let tem = b;
        b = b + a;
        a = tem;
    }
    return b;
}
