# booklog-cli

## Overview

**booklog-cli** is a CLI tool for your [Kyushu Institute of Technology (Kyutech) Library loan history (CSV)](https://www.lib.kyutech.ac.jp/library/ja/node/2061).

## Features

- Read loan history from a CSV file
- List borrowed books
- Search loan history

## Usage

```zsh
# List books
cargo run -- <CSV file> list
```

The input CSV file is expected to have the following columns:

`タイトル,貸出日,巻情報,著者,出版社,年月情報,資料ID,URL`

## Requirements

- Rust
- Cargo

---

Copyright (c) 2026 [@pantsman](https://github.com/pantsman-jp)
