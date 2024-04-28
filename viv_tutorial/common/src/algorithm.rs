pub fn is_prime(n: i32, i: i32) -> bool {
    //
    if n < 3 {
        return n == 2;
    };

    if (n % i) == 0 {
        return false;
    };

    if n < (i * i) {
        return true;
    };

    return is_prime(n, i + 1);
}

struct User {
    name: String,
    email: String,
    active: bool,
}
