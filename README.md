# k-nucleotide
Fully concurrent implementation of [k-nucleotide](http://benchmarksgame.alioth.debian.org/u64q/performance.php?test=knucleotide) written in Rust.

This is an experimental implementation aimed at making the most of CPU cores. It may NOT run faster than other versions with lower cpu usage.

## Comparisons
Intel(R) Xeon(R) CPU E5-26xx v2, 4 Cores:

||User|Real|CPU%|Memory|
|-|-|-|-|-|
|[C](http://benchmarksgame.alioth.debian.org/u64q/program.php?test=knucleotide&lang=gcc&id=1)|11.16s|4.45s|253%|130M|
|[Go](http://benchmarksgame.alioth.debian.org/u64q/program.php?test=knucleotide&lang=go&id=6)|30.74s|8.95s|344%|149M|
|[Rust](http://benchmarksgame.alioth.debian.org/u64q/program.php?test=knucleotide&lang=rust&id=2)|21.51s|8.33s|260%|165M|
|This Version|33.81s|9.14s|375%|251M|

Intel(R) Xeon(R) CPU E5-26xx v2, 16 Cores:

||User|Real|CPU%|Memory|
|-|-|-|-|-|
|[C](http://benchmarksgame.alioth.debian.org/u64q/program.php?test=knucleotide&lang=gcc&id=1)|11.03s|3.40s|328%|130M|
|[Go](http://benchmarksgame.alioth.debian.org/u64q/program.php?test=knucleotide&lang=go&id=6)|32.07s|6.12s|525%|150M|
|[Rust](http://benchmarksgame.alioth.debian.org/u64q/program.php?test=knucleotide&lang=rust&id=2)|19.17s|5.86s|328%|164M|
|This Version|281.34s|19.08s|1536%|251M|