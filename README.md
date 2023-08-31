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

### Manually

- Download the compressed archive and extract the executable somewhere into your path

### Using Scoop:

First add my bucket:

    scoop bucket add maciak https://github.com/maciakl/bucket

If you already have the bucked added:
    
    scoop update
    scoop install maciak/whatismyip

If you do not want to add my bucket:

    scoop install https://dmp.maciak.org/whatismyip.json
