pub fn loop_a() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 100 {
            break counter * 2;
        }
    };

    println!("\u{26EC} counter = {}", result);
}

pub fn loop_b() {
    //
    let mut count = 0;
    'counting_up: loop {
        println!("\u{26EC} count = {}", count);
        let mut remaining = 10;

        loop {
            println!("\u{26EC} remaining = {}", remaining);
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }
        count += 1;
    }

    println!("\u{26EC} End count = {}", count);
}

pub fn while_a() {
    // while loop
    let mut number = 3;

    while number != 0 {
        println!("\u{26EC} number = {}", number);
        number -= 1;
    }
    println!("\n\u{269E} LIFTOFF \u{269F}");
}

pub fn for_a() {
    // for loop
    let a = [10, 20, 30, 40, 50];
    for item in a {
        println!("\u{26EC} {}", item);
    }
}

pub fn for_b() {
    for item in (1..4).rev() {
        println!("\u{26EC} {}", item);
    }
    println!("\n\u{269E} LIFTOFF!!! \u{269F}");
}
