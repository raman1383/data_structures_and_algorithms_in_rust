///  Find Roots of a Quadratic Equation ax2 + bx + c = 0
pub fn find_root_quad_equation(a: f64, b: f64, c: f64) -> (f64, f64) {
    let r1: f64;
    let r2: f64;
    let rp: f64;
    let ip: f64;
    let d: f64 = (b as f64).powf(2.00) - (4.00 * (a * c));
    if d >= 0.00 {
        r1 = (-(b) + d.sqrt()) / (2.00 * a);
        r2 = (-(b) - d.sqrt()) / (2.00 * a);
        return (r1, r2);
    } else {
        rp = -(b) / (2.00 * a);
        ip = -(d).sqrt() / (2.00 * a);

        return (rp + a * (ip), rp - a * (ip));
    }
}

///Find the factorial of a number
pub fn find_factorial(n: f64) {
    let mut factorial: f64 = 1.00;
    let mut i: f64 = 1.00;

    while i != n {
        factorial = factorial * i;
        i = i + 1.00;
    }

    println!("{}", factorial)
}

/// Check whether a number is prime or not
pub fn prime_or_not(n: f64) -> bool {
    let mut flag: f64 = 1.00;
    let mut i: f64 = 2.00;

    while i != (n / 2.00) {
        if n % i == 0.00 {
            flag = 0.00;
            break;
        }
        i = i + 1.00;
    }
    if flag == 0.00 {
        return false;
    } else {
        return true;
    }
}
