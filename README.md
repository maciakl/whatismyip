# whatismyip

Quick and dirty tool to find your routable IP address.

Queries [ipify](https://ipify.org) service and prints the IP address to the terminal.

Written in Rust.

    Usage: whatismyip.exe [OPTIONS]

    Options:
    -l, --local    Get local ip address instead of the routable one
    -h, --help     Print help
    -V, --version  Print version

If you have multiple active network interfaces this tool will give information only about the default/preferred (ie the one your OS attempts to use first).

## Installing

Manually: 

- Download the zip and extract the executable

Using Scoop:

    scoop install https://dmp.maciak.org/whatismyip.json
