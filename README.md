# hybrid
Creates username permutations.

### usage

```
cargo build --release
cargo run --release -- names.txt usernames.txt
```

Inspired by [namemash.py](https://gist.github.com/superkojiman/11076951)

### example

To run, you need a file containing a full name (Name Surname) in each line, e.g.:
```
Galen Tyrol
Saul Tigh
Kara Thrace
```

Then, if you run the following it will generate `usernames.txt` as the output file:

```
cargo build --release
cargo run --release -- names.txt usernames.txt
```

`usernames.txt` file result:
```
gtyrol
g.tyrol
galentyrol
galen.tyrol
tyrolg
tyrol.g
tyrolgalen
tyrol.galen
gt
tg
stigh
s.tigh
saultigh
saul.tigh
tighs
tigh.s
tighsaul
tigh.saul
st
ts
kthrace
k.thrace
karathrace
kara.thrace
thracek
thrace.k
thracekara
thrace.kara
kt
tk
```
