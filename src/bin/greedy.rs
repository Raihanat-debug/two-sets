use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: u64 = input.trim().parse().unwrap();

    let total = n * (n + 1) / 2;

    if total % 2 != 0 {
        println!("NO");
        return;
    }

    println!("YES");

    let mut target = total / 2;

    let mut set1 = Vec::new();
    let mut set2 = Vec::new();

    for i in (1..=n).rev() {
        if i <= target {
            set1.push(i);
            target -= i;
        } else {
            set2.push(i);
        }
    }

    println!("{}", set1.len());

    for x in &set1 {
        print!("{} ", x);
    }
    println!();

    println!("{}", set2.len());

    for x in &set2 {
        print!("{} ", x);
    }
    println!();
}