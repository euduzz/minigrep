# minigrep

This is a small file text search command line tool made with Rust Language.

### How to install

At the moment, you can only use it by cloning this repo and running the `cargo` command directly in your machine.

```bash
$ cargo run word file.txt
```

### Testing

For the tests, you can use the `cargo` command and check for all the unit tests at once

```bash
$ cargo test
```

### To do

- [ ] Allow sentences as query
- [ ] Add support for flags
  - [ ] --help | -h to show instructions
  - [ ] --sensitive | --s to query with case sensitive on
  - [ ] --first | --f to show only the firs occurrence
- [ ] Create more scenario tests
  - [ ] case sensitive
  - [ ] full phrase
- [ ] Publish to homebre and apt repositories
