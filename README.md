# abridge

Abridge is a port of GNU's [word-list-compress](https://duckduckgo.com/?q=word-list-compress) to Rust.

`word-list-compress` is about 150 SLOC. Abridge is about 50 SLOC or 80 with the CLI. They perform identically under 
testing.

## Install

You'll need the Rust compiler and Cargo. The easiest way to do that is to get [rustup](https://rustup.rs/).

```shell
cargo install abridge
```

## Usage

See `abridge --help`.

## Examples

```shell
abridge -c < words.txt # compress words.txt
```

```shell
abridge --decompress < words.tzip # decompress words.tzip
```

```shell
abridge --compress < words.txt > words.tzip # compress words.txt and save to file
```

## Limitations

Abridge *compression* expects:

- Words are in alphabetical order
- Words are separated by newline

Abridge can *decompress* files compressed by `abridge` or `word-list-compress` alike.

## Testing

Run `cargo test`.

## License

Abridge is licensed under [GNU General Public License](https://www.gnu.org/licenses/gpl-3.0.en.html). 

