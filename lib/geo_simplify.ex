defmodule GeoSimplify do
  @moduledoc """
  This module represents how we found this in a production system. We have a large collection of
  data, 21,000
  """

  def simplify_csv do
    IO.puts("--rust call starts--")

    result = GeoNif.simplify()
    IO.puts("--rust call finished--")

    result
  end
end
