use rand::Rng;

fn main() {
    let mut rng_engine = rand::rng();                                                      //Random engine initialization
    let tiles: [char; 3] = ['g', 'w', 's'];                                         //Array with tiles, each char represents a tile. In this case: g = grass, w = water and s = sand
    let max_ran_range= 2;                                                                //Max random range

    let mut terrain_map: [[char; 3]; 3] = [
        ['-', '-', '-'],
        ['-', '-', '-'],
        ['-', '-', '-']
    ];

    for y in 0..terrain_map.len() {
        for x in 0..terrain_map[y].len() {
            let ranum = rng_engine.random_range(0..max_ran_range);
            terrain_map[y][x] = tiles[ranum];
        }
    }


    println!("Generated terrain: ");    
    for row in &terrain_map {
        for tile in row {
            print!(" {}", tile);
        }
        print!("\n");
    }

}