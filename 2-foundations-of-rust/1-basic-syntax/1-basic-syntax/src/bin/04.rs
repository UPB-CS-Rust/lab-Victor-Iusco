fn main() {
    let mut n :i32;
    for n in [10, 20, 30, 40] {
        let mult :i32 = if n < 25 {
            n * 4
        } else {
            n * 3
        };
        println!("{}", mult);
    }
}
