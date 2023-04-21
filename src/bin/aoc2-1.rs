use std::{fs, collections::HashMap};

fn main(){

    let input = fs::read_to_string("inputs/inp2.txt").unwrap();

    let shape_score_map = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    let game_score_map = HashMap::from([

        ("AX", 3), ("AY", 6), ("AZ", 0),
        ("BX", 0), ("BY", 3), ("BZ", 6),
        ("CX", 6), ("CY", 0), ("CZ", 3),
    ]);

    let mut total_score = 0;
    
    for line in input.lines(){
        let parts: Vec<&str> = line.split(' ').collect();
        let shape_score = shape_score_map.get(&parts[1]).unwrap();
        let game = parts[0].to_string()+parts[1];
        let game_score = game_score_map.get(&game[..]).unwrap();

        total_score += game_score + shape_score;
    }

    println!("Total score {}", total_score);




}
