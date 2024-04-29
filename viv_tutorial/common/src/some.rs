pub fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

pub fn check_optional(optional: Option<Box<i32>>) {
    match optional {
        Some(p) => println!("\n\u{269E} has value {p} \u{269F}"),
        None => println!("\n\u{269E} has no value \u{269F}"),
    }
}
