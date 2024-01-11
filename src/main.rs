use std::{io, fmt::Error};
use std::error;

struct Resistor {
    first_digit: u8,
    sec_digit: u8,
    third_digit: u8,
    mult: u8,
    tolerance: f32,
}

fn colors_to_values(input: String) -> Resistor {
    let split_vec: Vec<&str> = input.split(' ').collect();
    let mut v: Vec<u8> = Vec::new();
    for i in 0..=split_vec.len()-1 {
        match split_vec[i].to_lowercase().as_str() {
            "black" => v.push(0),
            "brown" => v.push(1),
            "red" => v.push(2),
            "orange" => v.push(3),
            "yellow" => v.push(4),
            "green" => v.push(5),
            "blue" => v.push(6),
            "violet" => v.push(7),
            "grey" => v.push(8),
            "white" => v.push(9),
            "gold" => v.push(10),
            "silver" => v.push(11),
            _ => v.push(0),
        }
    }

    let mut output = Resistor {
        first_digit: v[0],
        sec_digit: v[1],
        third_digit: if v.len() > 4 { v[2]} else {0},
        mult:  if v.len() > 4 { v[3]} else {v[2]},
        tolerance: if v.len() > 4 { v[4] as f32} else {v[3] as f32},
    };
    output.mult = 10_u8.pow(output.mult as u32);
    output.tolerance = color_to_tolerance(output.tolerance);
    output
}

fn color_to_tolerance(i:f32) -> f32 {
    let t = match i {
        2.0 => 1.0,
        3.0 => 2.0,
        5.0 => 0.5,
        6.0 => 0.25,
        7.0 => 0.1,
        _ => 0.0,
    };
    t
}

fn calc_resistor(i: Resistor) -> String {
    let r= (i.first_digit as f64
            +
            i.sec_digit as f64
            / 10. +
            i.third_digit as f64
            / 100.) *
            i.mult as f64;
    //let r = format_resistor(r);
    let s:String = format!("{:.3}Ohms +/-{}", r, i.tolerance);
    s            
}
/*
fn format_resistor(r: f32) -> (f32, char) {
    let r = r
} */

fn main() {
    let mut user_input = String::new();


    io::stdin()
        .read_line(&mut user_input)
        .expect("failed");

    let resistor = colors_to_values(user_input);
    
    println!("{}", calc_resistor(resistor));

}
