fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    let mut n = input.trim().parse::<i64>().unwrap();
    while n != 1 {
        print!("{n} ");
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = n * 3 + 1;
        }
    }
    println!("1")
}
