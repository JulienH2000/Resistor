use std::collections::HashMap;
use std::io;

pub fn get_user_input() -> String {
    let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("failed");
    user_input
}

fn get_color_units<'a, T>(i: T) -> Result<String, &'static str>
where
    T: PartialOrd + Copy + Into<f32>,
{   let index: f32 = i.try_into().unwrap();
    let color;
    match index {
        0_f32 => color = "black",
        1_f32 => color = "brown",
        2_f32 => color = "red",
        3_f32 => color = "orange",
        4_f32 => color = "yellow",
        5_f32 => color = "green",
        6_f32 => color = "blue",
        7_f32 => color = "purple",
        8_f32 => color = "grey",
        9_f32 => color = "white",
        _ => return Err("Invalid Multiplier !!!!")
    };
    Ok(color.to_string())
}

fn get_color_mult<'a, T>(i: T) -> Result<String, &'static str>
where
    T: PartialOrd + Copy + Into<f32>,
{   let index: f32 = i.try_into().unwrap();
    let color; 
    match index as f32 {
        0_f32 => color = "black",
        1e1 => color = "brown",
        1e2=> color = "red",
        1e3 => color = "orange",
        1e4 => color = "yellow",
        1e5 => color = "green",
        1e6 => color = "blue",
        1e7 => color = "purple",
        0.1 => color = "silver",
        0.01 => color = "gold",
        _ => return Err("Invalid Multiplier !!!!")
    };
    Ok(color.to_string())
}
pub struct Resistor {
    pub first_digit: Option<f32>,
    pub sec_digit: Option<f32>,
    pub third_digit: Option<f32>,
    pub mult: Option<f32>,
    pub tolerance: Option<f32>,
}

impl Resistor {
    pub fn as_array (&self) -> [&Option<f32>; 5] {
        [&self.first_digit,
        &self.sec_digit,
        &self.third_digit,
        &self.mult,
        &self.tolerance]
    }

    pub fn to_colors(self) -> Result<Colors, &'static str> {
        let output = Colors {
            first_color : match self.first_digit { 
                Some(x) => Some(match get_color_units(x) {
                    Ok(r) => r, 
                    Err(e) => return Err(e)}), 
                None => None },
            sec_color : match self.sec_digit { 
                Some(x) => Some(match get_color_units(x) {
                    Ok(r) => r, 
                    Err(e) => return Err(e)}), 
                None => Some(get_color_units(0_f32).unwrap())},
            third_color : match self.third_digit { 
                Some(x) => Some(match get_color_units(x) {
                    Ok(r) => r, 
                    Err(e) => return Err(e)}), 
                None => None },
            mult : match self.mult { 
                Some(x) => Some(match get_color_mult(x) {
                    Ok(r) => r, 
                    Err(e) => return Err(e)}), 
                None => None },
            tolerance : None,
        };
        Ok(output)

    }

    pub fn set_mult_for_colors (self) -> Result<Resistor, &'static str> {
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

    pub fn set_tolerance_for_colors (self) -> Result<Resistor, &'static str> {
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
    pub first_color: Option<String>,
    pub sec_color: Option<String>,
    pub third_color: Option<String>,
    pub mult: Option<String>,
    pub tolerance: Option<String>,
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

    if buf.len() >= 3 {
        return Err("Argument overflow !! Ignored");
    }
    let value = &buf[0];
    let unit;
    if buf.len()>1 {
        match si_to_multiplier(&buf[1]) {
            Ok(m) => unit =  m,
            Err(e) => return Err(e)
        }
    } else {
        unit = 1_f32;
    }

    let float_value;
    match value.replace(",", ".").parse::<f64>() {
        Ok(v) => float_value = v * (unit as f64),
        Err(_) => return Err("ParseIntError, Value Invalid !!")
    }

    if float_value.to_string()
        .chars()
        .filter(|&c| c != '.')
        .collect::<String>()
        .trim_end_matches("0")
        .len()
         > 3 {
            return Err("Too many significant unit values !!");
        }

    let extracted_value = extract_mantissa_exponent_f64(&float_value);
    let vec_mantissa: Vec<u32> = extracted_value.0
                                    .to_string()
                                    .chars()
                                    .filter(|c| c.is_ascii_digit() == true)
                                    .map(|d| d.to_digit(10).unwrap())
                                    .collect();

    let output = Resistor {
        first_digit: Some(vec_mantissa[0] as f32),
        sec_digit: if vec_mantissa.len() > 1  { Some(vec_mantissa[1] as f32) } else { None },
        third_digit: if vec_mantissa.len() > 2 { Some(vec_mantissa[2] as f32) } else { None },
        mult: Some(extracted_value.1 as f32),
        tolerance: None,
    };
    Ok(output)
}

fn extract_mantissa_exponent_f64(value: &f64) -> (u64, i32) {
    let value_string = String::from(format!("{}", value));
    let units = value_string
        .chars()
        .filter(|&c| c != '.')
        .collect::<String>();

    /*
    let multiplier = value_string
        .chars()
        .filter(|&c| c != '.')
        .filter(|&c| c == '0')
        .count();
    */
    let mut units = units.trim_end_matches("0").parse::<u64>().unwrap();

    //println!(" mult {}", multiplier);
    if units < 10 { units = units * 10 }
    let multiplier = (value / units as f64);

    (units, multiplier as i32)
}

fn si_to_multiplier(si: &str) -> Result<f32, &'static str> {
    match si {
        "n" => Ok(1e-9),
        "µ" => Ok(1e-6),
        "m" => Ok(1e-3),
        "k" | "K" => Ok(1e3),
        "M" => Ok(1e6),
        "G" => Ok(1e9),
        _ => Err("SI Unit Invalid !!")
    }

}
