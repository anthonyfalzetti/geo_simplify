defmodule GeoNif do
  @moduledoc """
  Binding to the forecasting Rust NIF
  """

  use Rustler, otp_app: :geo_simplify, crate: "geonif", mode: :release

  @doc """
  When the NIF is loaded, it will override this function.
  Apply the Ramer–Douglas–Peucker algorithm to a list to find the optimal subset
  of records for graphing.

  Inputs:
  - Records (a list of two element tuples)
  - Precision (a float)

  Output:
  - A list of indexes
  """

  def simplify(_records, _precision), do: :erlang.nif_error(:nif_not_loaded)
end
