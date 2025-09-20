# Stride

A simple command line program that creates a fullscreen overlay over all programs without taking keyboard focus.

I built this after safe eyes could just be switched to another workspace on tiling window managers.

## Installation

```sh
cargo install --git https://github.com/DhaiShah25/stride
```

## Usage

```
Usage: stride <MESSAGE> [DURATION] [INTERVAL]

Arguments:
  <MESSAGE>
  [DURATION]  [default: 20s]
  [INTERVAL]  [default: 20m]

Options:
  -h, --help     Print help
  -V, --version  Print version
```

The MESSAGE should be a string which will be displayed for DURATION at every INTERVAL. Both DURATION and INTERVAL have defaults shown above and use humantime to parse so you can easily write times without worrying about a precise format.

## Credits

This is based on the single window example from the iced_layershell project which is what I am using to create this overlay

Using clap for argument parsing was really nice to use.

I used humantime for easy parsing of the time for duration & interval
