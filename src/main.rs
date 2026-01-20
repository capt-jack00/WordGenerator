use rand::Rng;
use std::{thread, time};

fn main() {
    let mut rng = rand::rng();
    let tiles: [char; 3] = ['g', 'w', 's'];
    let mut terrain: [char; 10] = ['-'; 10];
    let max = 2;

    for i in 0..9{
        let mut ranum = rng.random_range(0..max);
        terrain[i] = tiles[ranum];
        println!("Random tile: {}", tiles[ranum]);
        thread::sleep(time::Duration::from_millis(500));                            //Sleep time in milliseconds to increase readabilty
        ranum = 0;
    }
}