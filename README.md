# peRISCope

Small tool written as a part of my work for bachelor thesis, used to visualize
the witness format of `btormc`.

# Build

To build this project you need to have Rust toolchain installed on your
computer. Check the [official website](https://www.rust-lang.org/tools/install)
for installation instructions.

After that, building is as simple as running:

```
cargo build
```

## Install

You can also install the program on your machine:

```
cargo install --path .
```

This will make the `periscope` command available in your terminal.

```
periscope --help
```

## Running

You can install the `periscope` as shown in the previous section. You can also
build and run directly using `cargo`:

```
cargo run -- <arguments_for_periscope>
```

For better periscope, you can use the `--release` flag:

```
cargo run --release -- <arguments_for_periscope>
```
