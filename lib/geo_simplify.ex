defmodule GeoSimplify do
  @moduledoc """
  This module represents how we found this in a production system. We have a large collection of
  data, 21,000 
  """

  def simplify_csv do
    path =
      "../test_data.csv"
      |> Path.expand(__DIR__)

    IO.puts("--Path.expand--")

    file = File.stream!(path)
    IO.puts("--File.stream--")

    csv = CSV.decode!(file)
    IO.puts("--CSV.decode--")

    data = Enum.map(csv, &to_rust/1)
    IO.puts("--to_rust--")

    result = GeoNif.simplify(data, 0.9)
    IO.puts("--GeoNif.simplify--")

    result
  end

  defp to_rust([elem1, elem2]) do
    x =
      elem1
      |> Float.parse()
      |> elem(0)

    y =
      elem2
      |> Integer.parse()
      |> elem(0)

    {x, y}
  end
end
