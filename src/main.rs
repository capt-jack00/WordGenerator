use rand::Rng;          //Library used to randomly generate numbers 
use colored::*;         //Library used to color output

fn main() {
    let mut rng_engine = rand::rng();                                                      //Random engine initialization
    let tiles: [char; 3] = ['g', 'w', 's'];                                         //Array with tiles, each char represents a tile. In this case: g = grass, w = water and s = sand

    //An array that holds colored ASCII square symbols
    let tiles_color: [colored::ColoredString; 3] = ["■".green(), "■".blue(), "■".yellow()];
    let max_ran_range= 3;                                                                //Max random range

    //A map that represents generated terrain
    let mut terrain_map: [[char; 3]; 3] = [
        ['-', '-', '-'],
        ['-', '-', '-'],
        ['-', '-', '-']
    ];

    //A loop that generates the terrain by randomly generating a number
    for y in 0..terrain_map.len() {
        for x in 0..terrain_map[y].len() {
            let ranum = rng_engine.random_range(0..max_ran_range);
            terrain_map[y][x] = tiles[ranum];
        }
    }

    //A loop that displays generated terrain and replaces the letters g, w, s with colored square ASCII symbols
    println!("Generated terrain: ");    
    for row in &terrain_map {
        for tile in row {
            match tile {
                'g' => print!("{}", tiles_color[0]),
                'w' => print!("{}", tiles_color[1]),
                's' => print!("{}", tiles_color[2]),
                _ => print!("{}", tile),
            }
        }
        println!();
    }

}








//TODO: There's probably a better way to store those colored ASCII symbols. 
//TODO: Figure out an algorithm that won't allow grass be generated nex to water and water next to grass. 