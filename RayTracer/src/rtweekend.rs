use rand::Rng;

pub fn random_double() -> f64 {
    rand::thread_rng().gen()
}

pub fn random_double_in_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

pub fn random_int_in_range(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min..max)
}
