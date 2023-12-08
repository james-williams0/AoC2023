use std::{
    fs::File,
    io::{prelude::*, self},
    path::Path
};

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let max_cubes = Cubeset { red: 12, green: 13, blue: 14 };
        let mut count: u32 = 0;
        let mut power_count: u32 = 0;
        for line in lines {
            if let Ok(input) = line {
                let game_id = input.split(':').next().unwrap().split(' ').last().unwrap().parse::<u32>().unwrap();
                let cubesets: Vec<Cubeset> = input.split(':').last().unwrap().split(';')
                    .map(|cube_set| parse_cubeset(cube_set)).collect();

                if cubesets.iter().all(|cube_set|
                    cube_set.red <= max_cubes.red &&
                    cube_set.green <= max_cubes.green &&
                    cube_set.blue <= max_cubes.blue) {
                        count = count + game_id;    
                    }

                let power =
                    cubesets.iter().max_by(|&set, &other_set| set.red.cmp(&other_set.red)).unwrap().red *
                    cubesets.iter().max_by(|&set, &other_set| set.green.cmp(&other_set.green)).unwrap().green *
                    cubesets.iter().max_by(|&set, &other_set| set.blue.cmp(&other_set.blue)).unwrap().blue;
                power_count = power_count + power;
            }
        }
        println!("Count: {}", count);
        println!("Power count: {}", power_count)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_cubeset(set: &str) -> Cubeset {
    let mut cubeset = Cubeset { ..Default::default() };

    for color_count_pair in set.split(',') {
        if color_count_pair.contains("red") {
            cubeset.red = color_count_pair[1..].split(' ').next().unwrap().parse::<u32>().unwrap();
        } else if color_count_pair.contains("green") {
            cubeset.green = color_count_pair[1..].split(' ').next().unwrap().parse::<u32>().unwrap();
        } else if color_count_pair.contains("blue") {
            cubeset.blue = color_count_pair[1..].split(' ').next().unwrap().parse::<u32>().unwrap();
        }
    }

    cubeset
}

struct Cubeset {
    red: u32,
    green: u32,
    blue: u32
}

impl Default for Cubeset {
    fn default() -> Cubeset {
        Cubeset {
            red: 0,
            green: 0,
            blue: 0
        }
    }
}