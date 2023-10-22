<div align="center">
    <h1>icalc</h1>
    <p>simple calculator</p>
</div>

# TODO
- equation solving algorithm(ie: x+5=10 => x=5, x^2+4x+3 => x=-3 or x=-1)

# building from git source
```sh
$ git clone https://codeberg.org/XDream8/icalc
$ cd icalc
$ cargo build --profile optimized
$ ./target/optimized/icalc
```

# examples
```sh
$ icalc "5+5"
$ icalc "16+sqrt(4)"
$ icalc "3^4+5^3"
$ icalc "log10(1000)+log2(512)"
```
