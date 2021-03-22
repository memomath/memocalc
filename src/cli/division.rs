use rand::Rng;
use rand::thread_rng;

pub fn num_gen(range: f32) -> ()  {
    thread_rng().gen_range(2.0 ..(range as f32 + 1.0));
}

pub fn operation(t: Vec<f32>) -> f32 {
    let mut n: f32 = 1.0;
    for i in t {
        n /= i;
    }
    return n as f32;
}

