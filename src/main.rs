use std::collections::HashMap;
use std::io;
//use std::error;

struct Resistor {
    first_digit: Option<u8>,
    sec_digit: Option<u8>,
    third_digit: Option<u8>,
    mult: Option<f32>,
    tolerance: Option<f32>,
}

fn colors_to_values(input: String) -> Result<Resistor, &'static str> {
    let split_vec: Result<Vec<&str>, &str> = input.split_whitespace().map(Ok).collect();
    let mut v: Vec<u8> = Vec::new();
    match split_vec{
        Ok(color) => 
        if color.len() < 4 {
            return Err("Not 4/5 rings color sequence !!");
        } else {
        for i in 0..=color.len()-1 {
            match color[i].to_lowercase().as_str() {
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
                _ => return Err("Invalid color !!"),
            }
        }},
        Err(error) => return Err(error),

    }

    let mut output = Resistor {
        first_digit: Some(v[0]),
        sec_digit: Some(v[1]),
        third_digit: if v.len() > 4 { Some(v[2])} else {None},
        mult:  if v.len() > 4 { Some(v[3] as f32)} else {Some(v[2] as f32)},
        tolerance: if v.len() > 4 { Some(v[4] as f32)} else {Some(v[3] as f32)},
    };
    output.mult = match output.mult {
        None => None,
        Some(m) => if m <= 7_f32{
                    Some(
                    (10_u32
                    .checked_pow(m as u32)
                    .expect("overflow")) as f32)
                } else if m == 10_f32 {
                    Some(0.1)
                } else if m == 11_f32 {
                    Some(0.01)
                } else {
                    None
                },
    };
    output.tolerance = color_to_tolerance(output.tolerance);
    Ok(output)
}

fn color_to_tolerance(i:Option<f32>) -> Option<f32> {
    let t = match i {
        None => None,
        Some(c) => Some(match c as i32 {
            1 => 1.0,
            2 => 2.0,
            5 => 0.5,
            6 => 0.25,
            7 => 0.1,
            10 => 5_f32,
            11 => 10_f32,
            _ => 0.0,
        },)
    };
    t
}

fn calc_resistor(i: Resistor) -> String {
    /*let r= (i.first_digit as f64
            +
            i.sec_digit as f64
            / 10. +
            i.third_digit as f64
            / 100.) *
            i.mult as f64;*/
    let mut values= HashMap::new();
    values.insert("first_digit", i.first_digit.unwrap_or(0) as f64);
    values.insert("sec_digit", i.sec_digit.unwrap_or(0) as f64);
    values.insert("third_digit", i.third_digit.unwrap_or(0) as f64);
    values.insert("mult", i.mult.unwrap_or(0_f32) as f64);
    values.insert("tolerance", i.tolerance.unwrap_or(0.0_f32) as f64);

    let r:f64;
    if None == i.third_digit {
        r= (
            values.get("first_digit").unwrap() *10_f64
            +
            values.get("sec_digit").unwrap() *1_f64
            ) *
            values.get("mult").unwrap();
    } else {
        r= (
            values.get("first_digit").unwrap() *100_f64
            +
            values.get("sec_digit").unwrap() *10_f64
            +
            values.get("third_digit").unwrap() *1_f64
            ) *
            values.get("mult").unwrap_or(&1_f64);
    }
    println!("{}", r);
    println!("{}", i.tolerance.unwrap());
    
    let result = match format_resistor(r) {
        Ok(r) => r,
        Err(_) => panic!("formatting failed !!")
    };
    let s:String = format!("{:.1} {} +/-{}%", result.0, result.1, values.get("tolerance").unwrap());
    s            
}

fn format_resistor(r: f64) -> Result<(f64, String), &'static str> {
    let output:(f64, String);
    let rounded = r as i32;
    let len = rounded.to_string();
    if len.len() <= 3 {
        output = (r, String::from("Ohms"));
    } else if len.len() <= 6 {
        output = (r/1000_f64, String::from("kOhms"));
    } else if len.len() <= 9 {
        output = (r/100000_f64, String::from("MOhms"));
    } else {
        output = (r, String::from("Ohms"));
    }
    Ok(output)
}

fn exit_check(i:&str) -> Result<bool, &'static str> {
    let split_vec: Vec<&str> = i.split_whitespace().collect();
    if split_vec.len() > 1 {
        return Ok(false);
    } else {
        match split_vec[0].to_lowercase().as_str() {
            "exit" => return Ok(true),
            _ => return Ok(false),
        }
    }
}

fn main() {
    println!("This software converts your resistor color code in Ohms,\nUse english colors, with spaces. This is case insensitive.\nType \"exit\" to quit");

    'input: loop {
    
        println!("Input your colors or exit:");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("failed");

        match exit_check(&user_input) {
            Ok(b) =>
                if b == true {
                    break 'input;
                },
            Err(e) => panic!("{}",e),
        }
    
        let resistor = colors_to_values(user_input);
        match resistor {
            Ok(r) => println!("{}", calc_resistor(r)),
            Err(e) => {
                println!("{}", e);
                continue},
        }
    
    }

}
