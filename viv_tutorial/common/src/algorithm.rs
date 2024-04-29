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

// struct User {
//     name: String,
//     email: String,
//     active: bool,
// }

pub fn fibnacci(number: usize) -> usize {
    fn fib_memo(n: usize, memo: &mut [Option<usize>]) -> usize {
        memo[n].unwrap_or_else(|| {
            let result = {
                if n > 1 {
                    fib_memo(n - 1, memo) + fib_memo(n - 2, memo)
                } else {
                    1
                }
            };
            memo[n] = Some(result);
            result
        })
    }
    fib_memo(number, &mut vec![None; number + 1])
}
