use std::collections::HashMap;
use std::io;
use std::fmt::Error;

struct Resistor {
    first_digit: Option<f32>,
    sec_digit: Option<f32>,
    third_digit: Option<f32>,
    mult: Option<f32>,
    tolerance: Option<f32>,
}

impl Resistor {
    fn as_array (&self) -> [&Option<f32>; 5] {
        [&self.first_digit,
        &self.sec_digit,
        &self.third_digit,
        &self.mult,
        &self.tolerance]
    }

    fn set_mult (self) -> Result<Resistor, &'static str> {
        let m = match self.mult {
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

        Ok(Resistor {
            mult: m,
            ..self          
            }
        )
    }

    fn set_tolerance (self) -> Result<Resistor, &'static str> {
        let t = match self.tolerance {
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
        Ok(Resistor {
            tolerance: t,
            ..self          
            }
        )

    }
}

struct Colors {
    first_color: Option<String>,
    sec_color: Option<String>,
    third_color: Option<String>,
    mult: Option<String>,
    tolerance: Option<String>,
}

impl Colors {
    fn as_array (&self) -> [&Option<String>; 5] {
        [&self.first_color,
        &self.sec_color,
        &self.third_color,
        &self.mult,
        &self.tolerance]
    }

    pub fn to_values (&self) -> Result<Resistor, &'static str>{
        let output = Resistor { 
            first_digit: match color_to_values(&self.first_color) {
                Ok(v) => Some(v),
                Err(_) => None,
            }, 
            sec_digit: match color_to_values(&self.sec_color) {
                Ok(v) => Some(v),
                Err(_) => None,
            },  
            third_digit: match color_to_values(&self.third_color) {
                Ok(v) => Some(v),
                Err(_) => None,
            }, 
            mult: match color_to_values(&self.mult) {
                Ok(v) => Some(v),
                Err(_) => None,
            }, 
            tolerance: match color_to_values(&self.tolerance) {
                Ok(v) => Some(v),
                Err(_) => None,
            }, };
        
        Ok(output)
        
        
    }
}

fn color_to_values(i:&Option<String>) -> Result<f32, &str> {
    match i {
       Some(v) => match v.to_lowercase().as_str() {
            "black" => Ok(0_f32),
            "brown" => Ok(1_f32),
            "red" => Ok(2_f32),
            "orange" => Ok(3_f32),
            "yellow" => Ok(4_f32),
            "green" => Ok(5_f32),
            "blue" => Ok(6_f32),
            "violet" => Ok(7_f32),
            "grey" => Ok(8_f32),
            "white" => Ok(9_f32),
            "gold" => Ok(10_f32),
            "silver" => Ok(11_f32),
            _ => return Err("Invalid color !!"),
            }
        None => Err("Empty Color !!"),
    }
} 


fn string_to_color (input: String) -> Result<Colors, &'static str> {
    let v: Vec<Option<String>> = input.split_whitespace().map(|s| Some(s.to_string())).collect();
    if v.len() == 4 {
        Ok(Colors {
            first_color: v[0].clone(),
            sec_color: v[1].clone(),
            third_color: None,
            mult: v[2].clone(),
            tolerance: v[3].clone(),
        })
    } else if  v.len () ==  5 {
        Ok(Colors {
            first_color: v[0].clone(),
            sec_color: v[1].clone(),
            third_color: v[2].clone(),
            mult: v[3].clone(),
            tolerance: v[4].clone(),
        })
    } else {
        return Err("Not 4/5 rings color sequence !!");
    }
    
}

fn calc_resistor(i: Resistor) -> String {
    let mut values= HashMap::new();
    values.insert("first_digit", i.first_digit.unwrap_or(0.) as f64);
    values.insert("sec_digit", i.sec_digit.unwrap_or(0.) as f64);
    values.insert("third_digit", i.third_digit.unwrap_or(0.) as f64);
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

fn main () {
    banner();
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

        let colors_input;
        match string_to_color(user_input) {
            Ok(r) => colors_input = r,
            Err(e) => {println!("{}",e);
                    continue;
            },
        }
        let resistor;
        match colors_input.to_values() {
            Ok(v) => resistor = v,
            Err(e) => {println!("{}",e);
                    continue;
            },
        }

        println!("{}",calc_resistor(resistor.set_mult().unwrap().set_tolerance().unwrap()));
    }
}

fn banner() {
    println!("
    ██████╗ ███████╗███████╗██╗███████╗████████╗ ██████╗ ██████╗ 
    ██╔══██╗██╔════╝██╔════╝██║██╔════╝╚══██╔══╝██╔═══██╗██╔══██╗
    ██████╔╝█████╗  ███████╗██║███████╗   ██║   ██║   ██║██████╔╝
    ██╔══██╗██╔══╝  ╚════██║██║╚════██║   ██║   ██║   ██║██╔══██╗
    ██║  ██║███████╗███████║██║███████║   ██║   ╚██████╔╝██║  ██║
    ╚═╝  ╚═╝╚══════╝╚══════╝╚═╝╚══════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝");

}
