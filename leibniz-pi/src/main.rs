fn main() {
    let mut pi = 0.0;
    let mut pos = true;

    for i in (1..10000000000_i64).step_by(2) {
        if pos {
            pi += 1.0 / (i as f64);
        } else {
            pi -= 1.0 / (i as f64);
        }

        pos = !pos;
    }

    pi *= 4.0;

    println!("{}", pi);
}
