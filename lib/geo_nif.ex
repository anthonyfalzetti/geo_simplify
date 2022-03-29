defmodule GeoNif do
  @moduledoc """
  Binding to the forecasting Rust NIF
  """

  use Rustler, otp_app: :geo_simplify, crate: "geonif", mode: :release

  @doc """
  When the NIF is loaded, it will override this function.

  Inputs: None

  Output: A list of indexes
  """

  def simplify(), do: :erlang.nif_error(:nif_not_loaded)
end
