# Rust

## Start

```bash
  $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  $ rustc --version
  $ cargo --version
  $ rustup update
  $ rustup doc
  $ cargo new hello-rust
  $ cd hello-rust
  $ cargo build
  $ cargo run
  $ cargo test
  $ cargo doc
  $ cargo publish
  $ cargo --help
  $ cargo check
  $ cargo login
  $ (paste token) : `create-io` saved


  # Cargo.toml : is the manifest file for Rust. It's where you keep metadata for your project, as well as dependencies.
  # src/main.rs : is where we'll write our application code.

```

## 소개

1. 빠르면서도 안전함 (High Performance)
2. 가비지 컬렉션 불필요 (Garbage Collection Not Necessary)
3. Concurrent Programming
4. Easy to Understand Errors
5. High-level language features without performance penalties
6. Program behaviors can be enforced at compile time
7. Built-in dependency management, similar to npm
8. Quickly growing ecosystem of libraries
9. Friendly & welcoming developer community
10. Memory safety
11. Userfull compiler output
12. Rich type system
13. Fast adoption in various branches

## References & Documents

* [doc.rust-lang](https://doc.rust-lang.org/cargo/getting-started/first-steps.html)
* [docs.rs](https://docs.rs/)
* [crates.io](https://crates.io/)

## Data Types

* Memory only stores binary data
    * Anythin can be represented in binary
* Program determines what the binary represents
* Basic types that are universally useful are provided by the language

* 메모리는 이진 데이터만 저장합니다
    * 모든 사물은 이진법으로 나타낼 수 있습니다
* 프로그램은 이진법이 무엇을 나타내는지 결정합니다
* 언어에 의해 보편적으로 유용한 기본 유형이 제공됩니다

## Basic Data Types

* Boolean
* Integer
* Double / Float
* Character
* String

## 변수 (Variable)

* Mutable, Immutable, const
    * 기본적으로 변수는 불변성으로 값이 할당 되면 변경할 수 없음
    * `mut` 키워드를 접두어로 사용하여 가변성으로 선언 할 수있음
    * `상수(const)` 는 런타임에 결정되는 값은 할당할 수 없으며
    * 오직 상수 리터럴만으로 할당할 수 있음
* 접두사
    * `i` : Signed
    * `u` : UnSigned
    * `isize` or `usize` : 컴퓨터 아키텍처에 따라 32bit 혹은 64bit 로 결정
* 정수
    * `i8` or `u8` : 8-bit
    * `i16` or `u16` : 16-bit
    * `i32` or `u32` : 32-bit
    * `isize` or `usize` : by Architecture 32 or 64-bit
* 부동 소수점
    * `f32` or `f64`
* 리터럴
    * `Decimal` : 1_000_000
    * `Hex` : 0xff
    * `Octal` : 0o77
    * `Binary` : ob1111_0000
    * `Byte (u8)` : b'A'
* Boolean
    * `bool`
* Character
    * `U+0000 ~ U+D7FF`
    * `U+E000 ~ U+10FFF`

```bash
    Max u32 :       4294967295
    Max u64 :       18446744073709551615
    Max usize :     18446744073709551615
    Max u128 :      340282366920938463463374607431768211455
    Max f32 :       340282350000000000000000000000000000000
    Max f64 :  179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
```

* Assign data to a temporary memeory location
    * Allows programmer to easily work with memory
* Can be set to any value & type
* Immutable[^1] by default, but can be mutable[^2]

[^1]: 불변, cannot be changed
[^2]: 가변, can be changed

## Variable Examples

```rust
  let two = 2;
  let hello = "hello"
  let j = 'j';
  let half_a = 0.5;
  let mut my_name = "Abc"; // 변형가능 형으로 선언 및 할당
  let quit_program = false;
  let half_b = half_a;
```

## Functions

* A way to enacpsulate program functionality, 프로그램 기능을 캡슐화하는 방법.
* Optionally accept data, 선택적으로 데이터 수락
* Optionally return data, 선택적으로 데이터 반환
* Utilized for code organization, 코드를 조직화 하는 기능
    * Also makes cod easier to read, 가독성 향상

* Anatomy of a function
    * `fn` : 키워드
    * `add` : Name
    * `(a: i32, b: i32)` : Parameters
    * `-> i32` : Return Type
    * `{ a + b }` : Body

```rust

  fn add(a: i32, b: i32) -> i32 {
    a + b
  }

  let x = add(1, 1);
  let y = add(3, 0);
  let z = add (x, 1);

```

## 프로젝트 생성

```bash
    cargo new viv_tutorial
```

## Build & Run

```bash
    cargo build && cargo run # puts the result -> target/dubug
 # or
 cargo build --release # puts the result -> target/release instead of target/debug
```

## VSCoce

* Snippets Example (rust.json)

```json
{
 "Viv println text": {
  "prefix": "pl",
  "body": [
   "println!(\"${1:Hello World!}\");$0"
  ],
  "description": "Viv println text"
 },
 "Viv println blank": {
  "prefix": "plb",
  "body": [
   "println!(${1});$0"
  ],
  "description": "Viv println blank"
 },
 "Viv println one": {
  "prefix": "pl1",
  "body": [
   "println!(\"${1:text}{}$2\", ${3:var});$0"
  ],
  "description": "Viv println one"
 },
 "Viv println two": {
  "prefix": "pl2",
  "body": [
   "println!(\"${1:text} {}$2 {}$3\", ${4:var1}, ${5:var2});$0"
  ],
  "description": "Viv println two"
 },
 "Viv Default Use": {
  "prefix": "#v",
  "body": [
   "#![allow(unused)]",
   "",
   "use std::io;",
   "use rand::Rng;",
   "use std::io:: {Write, BufReader, BufRead, ErrorKind};",
   "use std::fs::File;",
   "use std::cmp::Ordering;"
  ],
  "description": "Viv default use"
 }
}
```

### Ref. [Derek Banas](https://youtu.be/ygL_xcavzQ4)

### Ref. [Zero To Mastery](https://youtube.com/watch?v=lzKeecy4OmQ&si=p80MRuRI7B7CNgkT)
