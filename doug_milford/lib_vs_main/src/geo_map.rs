pub struct LatLong {
    pub lat: f32,
    pub long: f32,
}

pub fn get_hawaii_location() -> LatLong {
    LatLong {
        lat: 19.896,
        long: 155.583,
    }
}