defmodule GeoSimplify.MixProject do
  use Mix.Project

  def project do
    [
      app: :geo_simplify,
      version: "0.1.0",
      elixir: "~> 1.13",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger],
      mod: {GeoSimplify.Application, []}
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.24.0"},
      {:csv, "~> 2.4"}
    ]
  end
end
