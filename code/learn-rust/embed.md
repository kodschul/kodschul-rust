# Installation

1. clone repo

2. install `probe-rs`

```
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh


```

the following is installed:

cargo-embed
cargo-flash
probe-rs

3. find all stm32 boards

```
grep "stm32f7" -r --include="Cargo.toml"
```

chip feature: stm32f756zg

4. our board is **STM32F756ZGT6**

```bash

cargo install cargo-embed

```
