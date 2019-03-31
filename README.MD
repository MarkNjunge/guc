# GUC

A small application to convert a git url from SSH to HTTPS.

## Installation

Download the latest pre-built binary from [releases](https://github.com/MarkNjunge/guc/releases).

## Usage

`$ guc git@github.com:MarkNjunge/JustJava-Android.git`

or

`$ guc https://github.com/MarkNjunge/JustJava-Android.git`

## Development

```
git clone https://github.com/MarkNjunge/guc.git
cd guc

# Build
cargo build

# Test
cargo test

# Run
cargo run <url>
```
