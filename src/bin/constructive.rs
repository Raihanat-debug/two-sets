use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();

    if (n * (n + 1) / 2) % 2 == 1 {
        println!("NO");
        return;
    }

    println!("YES");

    let mut set1 = Vec::new();
    let mut set2 = Vec::new();

    if n % 4 == 0 {
        let mut i = 1;

        while i <= n {
            set1.push(i);
            set1.push(i + 3);

            set2.push(i + 1);
            set2.push(i + 2);

            i += 4;
        }
    } else {
        set1.push(1);
        set1.push(2);

        set2.push(3);

        let mut i = 4;

        while i <= n {
            set1.push(i);
            set1.push(i + 3);

            set2.push(i + 1);
            set2.push(i + 2);

            i += 4;
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
