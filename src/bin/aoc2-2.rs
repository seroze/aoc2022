use std::{fs, collections::HashMap};

fn main(){

    let input = fs::read_to_string("inputs/inp2.txt").unwrap();

    let shape_score_map = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    let game_score_map = HashMap::from([

        ("AX", 3), ("AY", 6), ("AZ", 0),
        ("BX", 0), ("BY", 3), ("BZ", 6),
        ("CX", 6), ("CY", 0), ("CZ", 3),
    ]);



    let strategy_score_map = HashMap::from([

        ("AX", "Z"), ("AY", "X"), ("AZ", "Y"),
        ("BX", "X"), ("BY", "Y"), ("BZ", "Z"),
        ("CX", "Y"), ("CY", "Z"), ("CZ", "X"),
    ]);


    let mut total_score = 0;
    
    for line in input.lines(){
        let parts: Vec<&str> = line.split(' ').collect();
//        println!("{:?}", parts);
        let cur_shape: String  = parts[0].to_string()+parts[1];
        let your_shape = strategy_score_map.get(cur_shape.as_str()).unwrap();
        let final_shape = parts[0].to_string()+your_shape;
        let shape_score = shape_score_map.get(your_shape).unwrap();
        //let game = parts[0].to_string()+shape;
        let game_score = game_score_map.get(final_shape.as_str()).unwrap();

        total_score += game_score + shape_score;
    }

    println!("Total score {}", total_score);




}
