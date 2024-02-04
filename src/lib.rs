use std::collections::HashMap;
use std::io;

pub fn get_user_input() -> String {
    let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("failed");
    user_input
}

fn get_color<T>(i: T) -> String
where
    T: PartialOrd + From<i32> + Copy,
{
    let color  = match i {
        1 => "brown",
        2 => "red",
        3 => "orange",
        4 => "yellow",
        5 => "green",
        6 => "blue",
        7 => "purple",
        8 => "grey",
        9 => "white",
    };
    color.to_string()
}
struct Resistor {
    first_digit: Option<f32>,
    sec_digit: Option<f32>,
    third_digit: Option<f32>,
    mult: Option<f32>,
    tolerance: Option<f32>,
}

impl Resistor {
    pub fn as_array (&self) -> [&Option<f32>; 5] {
        [&self.first_digit,
        &self.sec_digit,
        &self.third_digit,
        &self.mult,
        &self.tolerance]
    }

    pub fn to_colors (&self) -> Option<Colors> {
        let colors: Colors; 

        colors.first_color = match &self.first_digit {
            1 => "brown",
            2 => "red",
            3 => "orange",
            4 => "yellow",
            5 => "green",
            6 => "blue",
            7 => "purple",
            8 => "grey",
            9 => "white",
        };
                                    .collect();
        Some(colors)
    }

    pub fn set_mult (self) -> Result<Resistor, &'static str> {
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

    pub fn set_tolerance (self) -> Result<Resistor, &'static str> {
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

pub struct Colors {
    first_color: Option<String>,
    sec_color: Option<String>,
    third_color: Option<String>,
    mult: Option<String>,
    tolerance: Option<String>,
}

impl Colors {
    pub fn as_array (&self) -> [&Option<String>; 5] {
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

pub fn color_to_values(i:&Option<String>) -> Result<f32, &str> {
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


pub fn string_to_color (input: String) -> Result<Colors, &'static str> {
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

pub fn calc_resistor(i: Resistor) -> String {
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


pub fn string_to_values(input: String) -> Result<Resistor, &'static str> {
    let buf: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    if buf.len() >= 2 {
        println!("Argument overflow !! Ignored");
    }
    let value = &buf[0];
    let unit = si_to_multiplier(&buf[1]);
    let float_value = value.parse::<f64>().unwrap() * unit as f64;
    let extracted_value = extract_mantissa_exponent_f64(float_value);
    let vec_mantissa: Vec<u32> = extracted_value.0
                                    .to_string()
                                    .chars()
                                    .filter(|c| c.is_ascii_digit() == true)
                                    .map(|d| d.to_digit(10).unwrap())
                                    .collect();
    
    let output = Resistor {
        first_digit: Some(vec_mantissa[0] as f32),
        sec_digit: Some(vec_mantissa[1] as f32),
        third_digit: if vec_mantissa.len() <= 4 { Some(vec_mantissa[2] as f32) } else { None },
        mult: Some(extracted_value.1 as f32),
        tolerance: None,
    };
    Ok(output)
}

fn extract_mantissa_exponent_f64(value: f64) -> (u64, i32) {
    let bits = value.to_bits();
    let mantissa = bits & ((1u64 << 52) - 1);
    let exponent = ((bits >> 52) & 0x7FF) as i32 - 1023;
    (mantissa, exponent)
}

fn si_to_multiplier(si: &str) -> f32 {
    match si {
        "n" => 0.000000001,
        "µ" => 0.000001,
        "m" => 0.001,
        "k" => 1000_f32,
        "M" => 1000000_f32,
        "G" => 1000000000_f32,
    }

}
