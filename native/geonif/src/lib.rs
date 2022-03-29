rustler::init!("Elixir.GeoNif", [simplify]);

#[rustler::nif(schedule = "DirtyCpu")]
pub fn simplify() -> Vec<usize> {
    println!("**rust: starting**");
    let result = recursive_function(5000);
    println!("Length of empty result: {}", result.len());
    println!("**rust done**");
    result
}

fn recursive_function(counter: u64) -> Vec<usize> {
    if counter % 1000 == 0 {
        println!("{}", counter); // Force the stack to grow
    }
    if counter == 0 {
        vec![]
    } else {
        recursive_function(counter - 1)
    }
}
