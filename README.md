# ipv6_sort
this repo contain utility written in rust for sorting ipv6 addresses reading them from file

How-to build:

1. clone source from this repo
2. cd ip_ipv6
3. cargo build --release

Issues:
For now util just simply ignored not correct lines from file
For now no checking is file not empty

Further work:
add default behavior: read stdin, write to stdout
add verbose

USAGE:
    ipv6_sort --input <INPUT FILE> --output <OUTPUT FILE>

