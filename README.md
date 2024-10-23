# whatismyip

Quick and dirty tool to find your routable IP address.

Queries [ipify](https://ipify.org) service and prints the IP address to the terminal.

## Usage

    Usage: whatismyip.exe [OPTIONS]

    Options:
    -l, --local    Get local ip address instead of the routable one
    -h, --help     Print help
    -V, --version  Print version

If you have multiple active network interfaces this tool will give information only about the default/preferred (ie the one your OS attempts to use first).

## Installing

Install via `cargo`:

     cargo install https://github.com/maciakl/whatismyip/ 
 
 On Windows, this tool is also distributed via `scoop` (see [scoop.sh](https://scoop.sh)).

 First, you need to add my bucket:

    scoop bucket add maciak https://github.com/maciakl/bucket
    scoop update

 Next simply run:
 
    scoop install whatismyip
