use std::ops::Sub;

rustler::init!("Elixir.GeoNif", [simplify]);

#[rustler::nif(schedule = "DirtyCpu")]
pub fn simplify(records: Vec<(f64, u64)>, precision: f64) -> Vec<usize> {
    println!("**simplify** num_records: {}", records.len());
    let v = records
        .iter()
        .map(|&x| Coordinate {
            x: x.0,
            y: x.1 as f64,
        })
        .collect::<Vec<_>>();
    println!("**collected**");
    let line_string = LineString(v);
    println!("**LineString**");
    let result = line_string.simplify_idx(&precision);
    println!("**simplified**");
    result
}

// Everything below here was copied in from the GEO crate with only changes to some
// generics made to keep the code more concise

#[derive(Clone, PartialEq, Copy)]
pub struct Coordinate {
    x: f64,
    y: f64,
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Coordinate {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Line {
    pub start: Coordinate,
    pub end: Coordinate,
}

impl Line {
    pub fn new(start: Coordinate, end: Coordinate) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
        }
    }

    /// Calculate the difference in coordinates (Δx, Δy).
    pub fn delta(&self) -> Coordinate {
        self.end - self.start
    }
    pub fn dx(&self) -> f64 {
        self.delta().x
    }
    pub fn dy(&self) -> f64 {
        self.delta().y
    }
}

fn line_euclidean_length(line: &Line) -> f64 {
    line.dx().hypot(line.dy())
}

struct LineString(Vec<Coordinate>);

#[derive(Clone, Copy)]
struct RdpIndex {
    index: usize,
    coord: Coordinate,
}

impl LineString {
    fn simplify_idx(&self, epsilon: &f64) -> Vec<usize> {
        calculate_rdp_indices(
            &self
                .0
                .iter()
                .enumerate()
                .map(|(idx, coord)| RdpIndex {
                    index: idx,
                    coord: *coord,
                })
                .collect::<Vec<RdpIndex>>(),
            epsilon,
        )
    }
}

// Wrapper for the RDP algorithm, returning simplified point indices
fn calculate_rdp_indices(rdp_indices: &[RdpIndex], epsilon: &f64) -> Vec<usize> {
    if *epsilon <= 0.0 {
        return rdp_indices
            .iter()
            .map(|rdp_index| rdp_index.index)
            .collect();
    }
    compute_rdp(rdp_indices, *epsilon)
        .into_iter()
        .map(|rdpindex| rdpindex.index)
        .collect::<Vec<usize>>()
}

// Ramer–Douglas-Peucker line simplification algorithm
// This function returns both the retained points, and their indices in the original geometry,
// for more flexible use by FFI implementers
fn compute_rdp(rdp_indices: &[RdpIndex], epsilon: f64) -> Vec<RdpIndex>
where
{
    if rdp_indices.is_empty() {
        return vec![];
    }

    let first = rdp_indices[0];
    let last = rdp_indices[rdp_indices.len() - 1];
    let first_last_line = Line::new(first.coord, last.coord);

    // Find the farthest `RdpIndex` from `first_last_line`
    let (farthest_index, farthest_distance) = rdp_indices
        .iter()
        .enumerate()
        .take(rdp_indices.len() - 1) // Don't include the last index
        .skip(1) // Don't include the first index
        .map(|(index, rdp_index)| {
            (
                index,
                line_segment_distance(
                    &rdp_index.coord,
                    &first_last_line.start,
                    &first_last_line.end,
                ),
            )
        })
        .fold(
            (0usize, 0.0),
            |(farthest_index, farthest_distance), (index, distance)| {
                if distance > farthest_distance {
                    (index, distance)
                } else {
                    (farthest_index, farthest_distance)
                }
            },
        );

    if farthest_distance > epsilon {
        // The farthest index was larger than epsilon, so we will recursively simplify subsegments
        // split by the farthest index.
        let mut intermediate = compute_rdp(&rdp_indices[..=farthest_index], epsilon);
        intermediate.pop(); // Don't include the farthest index twice
        intermediate.extend_from_slice(&compute_rdp(&rdp_indices[farthest_index..], epsilon));
        intermediate
    } else {
        // The farthest index was less than or equal to epsilon, so we will retain only the first
        // and last indices, resulting in the indices inbetween getting culled.
        vec![first, last]
    }
}

pub fn line_segment_distance(point: &Coordinate, start: &Coordinate, end: &Coordinate) -> f64 {
    if start == end {
        return line_euclidean_length(&Line::new(*point, *start));
    }
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let r = ((point.x - start.x) * dx + (point.y - start.y) * dy) / (dx.powi(2) + dy.powi(2));
    if r <= 0.0 {
        return line_euclidean_length(&Line::new(*point, *start));
    }
    if r >= 1.0 {
        return line_euclidean_length(&Line::new(*point, *end));
    }
    let s = ((start.y - point.y) * dx - (start.x - point.x) * dy) / (dx * dx + dy * dy);
    s.abs() * dx.hypot(dy)
}
