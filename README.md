# Disco CLI

:fire: Generate recommendations from CSV files

- Supports user-based and item-based recommendations
- Works with explicit and implicit feedback
- Uses high-performance matrix factorization

Also available for [Rust](https://github.com/ankane/disco-rust) and [Ruby](https://github.com/ankane/disco)

[![Build Status](https://github.com/ankane/disco-cli/workflows/build/badge.svg?branch=master)](https://github.com/ankane/disco-cli/actions)

## Installation

Download the latest version:

- Mac - [x86_64](https://github.com/ankane/disco-cli/releases/download/v0.1.0/disco-0.1.0-x86_64-apple-darwin.tar.gz) or [arm64](https://github.com/ankane/disco-cli/releases/download/v0.1.0/disco-0.1.0-aarch64-apple-darwin.tar.gz)
- Linux - [x86_64](https://github.com/ankane/disco-cli/releases/download/v0.1.0/disco-0.1.0-x86_64-unknown-linux-gnu.tar.gz)
- Windows - [x86_64](https://github.com/ankane/disco-cli/releases/download/v0.1.0/disco-0.1.0-x86_64-pc-windows-msvc.zip)

With Homebrew, run:

```sh
brew install ankane/brew/disco
```

With Cargo, run:

```sh
cargo install disco-cli
```

## Quickstart

Download the MovieLens 100k dataset and generate item-based recommendations

```sh
disco download movielens-100k
disco item-recs movielens-100k.csv output.csv --factors 20
grep "^Star Wars" output.csv
```

## How to Use

### Data

Create a CSV file with your data. If users rate items directly, this is known as explicit feedback. The CSV should have three columns: `user_id`, `item_id`, and `rating`.

```csv
user_id,item_id,rating
1,post1,5
1,post2,3.5
2,post1,4
```

If users don’t rate items directly (for instance, they’re purchasing items or reading posts), this is known as implicit feedback. Use `value` instead of `rating` and a value like number of purchases, number of page views, or just `1`.

```csv
user_id,item_id,value
1,post1,1
1,post2,1
2,post1,1
```

Each `user_id`/`item_id` combination should only appear once.

### User-based Recommendations

Generate user-based recommendations - “users like you also liked”

```sh
disco user-recs data.csv output.csv
```

This creates a CSV with `user_id`, `recommended_item_id`, and `score` columns.

### Item-based Recommendations

Generate item-based recommendations - “users who liked this item also liked”

```sh
disco item-recs data.csv output.csv
```

This creates a CSV with `item_id`, `recommended_item_id`, and `score` columns.

### Similar Users

Generate similar users

```sh
disco similar-users data.csv output.csv
```

This creates a CSV with `user_id`, `similar_user_id`, and `score` columns.

## Algorithms

Disco uses high-performance matrix factorization.

- For explicit feedback, it uses the [stochastic gradient method with twin learners](https://www.csie.ntu.edu.tw/~cjlin/papers/libmf/mf_adaptive_pakdd.pdf)
- For implicit feedback, it uses the [conjugate gradient method](https://www.benfrederickson.com/fast-implicit-matrix-factorization/)

Specify the number of factors and iterations

```sh
disco ... --factors 8 --iterations 20
```

## Options

Specify the number of recommendations for each user or item

```sh
disco ... --count 10
```

## Datasets

Download a dataset

```sh
disco download movielens-100k
```

Supported datasets are:

- movielens-100k
- movielens-1m
- movielens-25m
- movielens-latest-small [unreleased]
- movielens-latest [unreleased]

## History

View the [changelog](https://github.com/ankane/disco-cli/blob/master/CHANGELOG.md)

## Contributing

Everyone is encouraged to help improve this project. Here are a few ways you can help:

- [Report bugs](https://github.com/ankane/disco-cli/issues)
- Fix bugs and [submit pull requests](https://github.com/ankane/disco-cli/pulls)
- Write, clarify, or fix documentation
- Suggest or add new features

To get started with development:

```sh
git clone https://github.com/ankane/disco-cli.git
cd disco-cli
cargo run
```
