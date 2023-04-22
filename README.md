# Rust

## 소개
1. 빠르면서도 안전함 (High Performance)
2. 가비지 컬렉션 불필요 (Garbage Collection Not Necessary)
3. Concurrent Programming
4. Easy to Understand Errors


## 프로젝트 생성

```bash
    $ cargo new hello-world
```

## References & Documents

> [docs.rs](https://docs.rs/)
>
> [crates.io](https://crates.io/)

## Build & Run

```bash
    cargo build && cargo run
```

## 변수 (Variable)

- Mutable, Immutable, const 
  - 기본변수는 불변성으로 값이 할당 되면 변경할 수 없음
  - `mut` 키워드를 접두어로 사용하여 가변성으로 선언 할 수있음
  - `상수(const)` 는 런타임에 결정되는 값은 할당할 수 없으며
  - 오직 상수 리터럴만으로 할당할 수 있음 
- 접두사
  - `i` : Signed
  - `u` : UnSigned
  - `isize` or `usize` : 컴퓨터 아키텍처에 따라 32bit 혹은 64bit 로 결정
- 정수
  - `i8` or `u8` : 8-bit
  - `i16` or `u16` : 16-bit
  - `i32` or `u32` : 32-bit
  - `isize` or `usize` : by Architecture 32 or 64-bit
- 부동 소수점
  - `f32` or `f64`
- 리터럴
  - `Decimal` : 1_000_000
  - `Hex` : 0xff
  - `Octal` : 0o77
  - `Binary` : ob1111_0000
  - `Byte (u8)` : b'A'
- Boolean
  - `bool`  
- Character
  - `U+0000 ~ U+D7FF`
  - `U+E000 ~ U+10FFF`

```bash
    Max u32 :       4294967295
    Max u64 :       18446744073709551615
    Max usize :     18446744073709551615
    Max u128 :      340282366920938463463374607431768211455
    Max f32 :       340282350000000000000000000000000000000
    Max f64 :       179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
```

## VSCoce

- Snippets Example (rust.json)

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
