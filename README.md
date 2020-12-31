# UUID

`UUID` is a command-line tool to generate UUID v4 value.

## Getting Started

`UUID` tool arouse from the need to generate few UUID values for use in DevOps processes and other CLI tools. The `Rust` implementation is my first attempt at [Rust command-line apps](https://www.rust-lang.org/what/cli) coding as the small single-executable footprint and the cross-platform support are significant benefits.

### Prerequisites

* Rust

### Build

* Clone this repository
* Run `cargo build --release`

### Usage

```text
uuid [number of UUIDs to generate] [output format]
```

* Number of UUIDs to generate:
  * When invalid or not present, the default value is `1`
* Output Format:
  * `-a` display in all formats (default)
  * `-b` bytes
  * `-h` hyphenated lowercase
  * `-H` hyphenated uppercase
  * `-s` simple lowercase
  * `-S` simple uppercase
  * `-u` urn lowercase
  * `-U` urn uppercase
