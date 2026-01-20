use rand::Rng;
use std::{thread, time};

fn main() {
    let mut rng = rand::rng();
    let tiles: [char; 3] = ['g', 'w', 's'];
    let max = 2;
    while true{
        println!("Random tile: {}", tiles[rng.random_range(0..max)]);
        thread::sleep(time::Duration::from_millis(500));
    }
}