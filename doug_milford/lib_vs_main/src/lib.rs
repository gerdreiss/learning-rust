use crate::geo_map::get_hawaii_location;

mod geo_map;

pub fn lib_func() {
    let hawaii_loc = get_hawaii_location();
    println!("Hawaii loc: {} {}", hawaii_loc.lat, hawaii_loc.long);
}
