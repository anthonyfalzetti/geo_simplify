extern crate geo;

use geo::algorithm::simplify::SimplifyIdx;

use geo::{Coordinate, LineString};

rustler::init!("Elixir.GeoNif", [simplify]);

#[rustler::nif(schedule = "DirtyCpu")]
pub fn simplify(records: Vec<(f64, u64)>, precision: f64) -> Vec<usize> {
    let v = records
        .iter()
        .map(|&x| Coordinate {
            x: x.0,
            y: x.1 as f64,
        })
        .collect::<Vec<_>>();
    let line_string = LineString(v);
    line_string.simplify_idx(&precision)
}
