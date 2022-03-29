# GeoSimplify

This Repo is a small reproduction of a SegFault or "bus error" that occured while
using rustler. Curiously this error does not occur if you use the Rust in a Rust
project, only while using Rustler.

## Steps to reproduce

1. clone this repo
2. run `mix deps.get`
3. run `mix test`

And you should see the following output:

```
$ mix test
--rust call starts--
**rust: starting**
5000
4000
3000
2000
zsh: bus error  mix test
```

You can also run it interactively (this will bork your current terminal buffer):

```
iex -S mix

iex(1)> GeoSimplify.simplify_csv()
--rust call starts--
**rust: starting**
                  5000
                      4000
                          3000
                              2000
                                  zsh: bus error  iex -S mix
```
