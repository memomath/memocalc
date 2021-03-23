use rand::thread_rng;
use rand::Rng;

pub fn num_gen(range: i32) -> i32 {
    thread_rng().gen_range(1..(range + 1))
}

pub fn operation(t: Vec<i32>) -> i32 {
    let n = t[1..].iter().fold(t[0], |acc, &x| acc / x);
    return n;
}
