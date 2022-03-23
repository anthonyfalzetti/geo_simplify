# GeoSimplify

This repo is used to reproduce a segfault error while using the geo crate.
Crash report added

## Steps to reproduce
1. clone this repo
2. run `mix deps.get`
3. run `mix test`

```
epn-afalzetti:geo_simplify  $ mix test
Compiling 1 file (.ex)
Bus error: 10
```

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `geo_simplify` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:geo_simplify, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at <https://hexdocs.pm/geo_simplify>.
