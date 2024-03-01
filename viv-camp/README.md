# viv-camp

## Start

- Pointer sized integer type, matches size of a word in given platform
- Numbers - usize & isize

## What is a word? (워드)

>- Processor does NOT read 1byte a a time from memory, read 1 word at a time.
>- In a 32-bit processor it can access 4 bytes (32 bits) at a time.
    >- 32 bit architecture, word => 4bytes
>- In a 64-bit processor it can access 8 bytes (64 bits) at a time.
>- usize gives you the guaratee to be always big enough to hold any pointer or any offset in a data structure

## Floating Point (부동 소수점)

>- f32 - size of 32 bits
>- f64 - size of 64 bits
>- Representation according to IEEE-754 specification
