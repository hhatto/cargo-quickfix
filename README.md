# cargo-quickfix

## Installation and Setup

```
$ git clone https://github.com/hhatto/cargo-quickfix.git
$ cd cargo-quickfix
$ cargo install --force
```

### for vimmer

```
$ mkdir -p PATH_TO_VIM_SYNTASTIC/syntax_checkers/rust
$ cp -p PATH_TO_VIM_SYNTASTIC/syntax_checkers/rust/
$ echo "let g:syntastic_rust_checkers = ['cargo']" >> $HOME/.vimrc
```

## Usage

```
$ cargo quickfix
src/main.rs:3:5: warning: unused import: `std::thread`, #[warn(unused_imports)] on by default
```
