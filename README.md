# GeoSimplify

This Repo is a small reproduction of a SegFault or "bus error" that occured while
using rustler. Curiously this error does not occur if you use the Rust in a Rust
project, only while using Rustler.

The name of this repo refers to the offending code being original contained in the
Geo crate: https://crates.io/crates/geo , but all the necessary code has been pulled
into this repo.

## Steps to reproduce

1. clone this repo
2. run `mix deps.get`
3. run `mix test`

And you should see the following output:

```
$ mix test
--Path.expand--
--File.stream--
--CSV.decode--
--to_rust--
**simplify** num_records: 21600
**collected**
**LineString**
[1]    12818 bus error  mix test
```

You can also run it interactively (this will bork your current terminal buffer):

```
iex -S mix

iex(1)> GeoSimplify.simplify_csv()

--Path.expand--
--File.stream--
--CSV.decode--
--to_rust--
**simplify** num_records: 21600
                               **collected**
                                            **LineString**
                                                          [1]    13163 bus error  iex -S mix
                                                                                            %

```
