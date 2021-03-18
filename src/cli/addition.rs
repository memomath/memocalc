use rand::Rng;
use rand::thread_rng;


pub fn num_gen(range: i32) -> i32 {
    thread_rng().gen_range(1..(range+1))
}

pub fn operation(t: Vec<i32>) -> i32 {
    return t.iter().sum();
}

