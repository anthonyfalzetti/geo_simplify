defmodule GeoSimplify do
  @moduledoc """
  Documentation for `GeoSimplify`.
  """

  def simplify_csv do
    "../bad_data.csv"
    |> Path.expand(__DIR__)
    |> File.stream!()
    |> CSV.decode!()
    |> Enum.map(&restructure/1)
    |> GeoNif.simplify(0.9)
  end

  defp restructure([elem1, elem2]) do
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
