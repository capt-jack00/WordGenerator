use rand::Rng;
use std::{thread, time};

fn main() {
    let mut rng = rand::rng();                                                      //Random engine initialization
    let tiles: [char; 3] = ['g', 'w', 's'];                                         //Array with tiles, each char represents a tile. In this case: g = grass, w = water and s = sand
    let mut terrain: [char; 10] = ['-'; 10];                                        //Terrain array with max capacity of 10 elements. At first it's filled with '-' and as the program runs it's getting filled with chars representing tiles
    let max = 2;                                                                    //Max random range

    for i in 0..9{
        let mut ranum = rng.random_range(0..max);                                   //Generates a random number in range between 0 and max 
        terrain[i] = tiles[ranum];                                                  //Saves the generated number to terrain array on position i
        println!("Random tile: {}", tiles[ranum]);                                  
        thread::sleep(time::Duration::from_millis(500));                            //Sleep time in milliseconds to increase readabilty
        ranum = 0;                                                                  //Zeroing the ranum, just in case
    }
}