# Start

## Ceate new Workspace

>- make directory

```bash
    mkdir viv
    cd viv
```

>- make toml (root)

```toml
[workspace]
resolver = "2"
members = [
    "viv",
]

```

>- make binary

```bash
    cargo new viv
```

>- make second package

```toml
[workspace]

members = [
    "viv",
    "bootcamp",
]
```

```bash
    cargo new bootcamp --lib
```

>- add defendency ("viv/Cargo.toml")

```toml
    [dependencies]
    bootcamp = { path = "../bootcamp"}
```
