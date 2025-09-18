# hb

A CLI/TUI for [Homebank](http://homebank.free.fr/).

> [!Note]
> Development of `hb` has moved to [this new location](https://gitlab.com/jrhawley/hb/) and this repository on GitHub is archived.

## Installation

On Windows, Linux, or macOS, install with [Cargo](https://doc.rust-lang.org/cargo/).

```shell
cargo install --git https://github.com/jrhawley/hb.git
```

## Usage

```shell
> hb -h
Query and operate on your HomeBank database from the command line.

USAGE:
    hb [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <path>    Path to hb configuration file

SUBCOMMANDS:
    budget    Look at your category budgets [aliases: b]
    help     Prints this message or the help of the given subcommand(s)
    query    Perform a query on the HomeBank database [aliases: q]    
    sum      Calculate a sum of transactions in a query [aliases: t, s]
```

## How it works

See [this blog post](https://jrhawley.ca/2022/04/14/homebank-cli) for details about the motivation and design implementation of `hb`.

### Customized configuration

A configuration file will automatically be loaded from your user's application settings, if one exists.
`hb` uses the [`dirs-next`](https://docs.rs/dirs-next/) crate to achieve this, which follows the [expected conventions](https://docs.rs/dirs-next/latest/dirs_next/fn.config_dir.html) in each operating system.

| Operating system | Configuration location                                    |
| ---------------- | --------------------------------------------------------- |
| macOS            | `$HOME/Library/Application Support/quill/config.toml`     |
| Linux            | `$HOME/.config/quill/config.toml`                         |
| Windows          | `C:\\Users\\<User>\\AppData\\Roaming\\quill\\config.toml` |

