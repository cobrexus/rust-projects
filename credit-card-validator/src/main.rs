fn main() {
    let mut n = 5893804115457289_i64;
    let check_digit = n % 10;
    n /= 10;

    let mut n = n
        .to_string()
        .chars()
        .rev()
        .filter_map(|c| c.to_string().parse::<i64>().ok())
        .collect::<Vec<_>>();

    n.iter_mut().enumerate().for_each(|(i, d)| {
        if i % 2 == 0 {
            *d *= 2;

            if *d > 9 {
                *d -= 9;
            }
        }
    });

    n.push(check_digit);

    let sum = n.iter().sum::<i64>();

    if sum % 10 == 0 {
        println!("valid");
    } else {
        println!("invalid");
    }
}
