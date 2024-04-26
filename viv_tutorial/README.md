# Rust

```bash
    cargo new viv_tutorial --bin
    cargo new snippet --lib
    cargo build
    cargo check # 실행파일은 생성하지 않음.
    cargo --version
    cargo new hello_world --bin
    cargo build
    cargo check
    cargo run -q
    restup update
    rustc --version
    rustup doc
```

## 매크로(Macro)

> 다른 코드를 작성하는 코드.
> 메타 그로그래밍

- 선언적 매크로 (declarative macros)
- 절차적 매크로 (procedural macros)
- macros by example
- maxro_rules!

- proc_macro: 러스트 코드를 문자열로 변환하는 기능.
- syn: 문자열로 변환한 러스트 코드를 연산을 수행하기 위한 자료 구조로 파싱함.
- quote: syn 자료 구조를 러스트 코드로 복원하는 역할을 함.
