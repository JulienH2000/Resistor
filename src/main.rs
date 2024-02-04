use std::collections::HashMap;
use std::io;
use std::fmt::Error;
use resistor::*;



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

enum Mode {
    Colors,
    Value,
}

fn menu_check (i:&str) -> Option<Mode> {
    let buff: Vec<&str> = i.split_whitespace().collect();
    match buff[0].to_lowercase().as_str() {
        "colors" | "color" => return Some(Mode::Colors),
        "values" | "value" => return Some(Mode::Value),
        _ => None,
    }
}

fn color_mode () {
    'input: loop {

        println!("Input your colors or exit:");
        let user_input = get_user_input();
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

fn value_mode() {
    println!("entering value mode...\n");
    'tolerance: loop {
        println!("Enable tolerance input ?\n");
        let user_input = get_user_input();
    }
    'input: loop {

        println!("Input your value or exit:");
        let user_input = get_user_input();
        match exit_check(&user_input) {
            Ok(b) =>
                if b == true {
                    break 'input;
                },
            Err(e) => panic!("{}",e),
        }

        let value_input;
        match value_input.to
    
    }
}

fn main () {
    banner();
    println!("This software converts your resistor color code in Ohms,\nUse english colors, with spaces. This is case insensitive.\nType \"exit\" to quit");

    let mut mode: Mode;

    'menu: loop {
        println!("Select input mode: Colors / Value, or exit");
        let user_input = get_user_input();
            match menu_check(&input_select) {
                Some(m) => { break 'menu; mode = m} ,
                None => continue,
            }
            match exit_check(&user_input) {
                Ok(b) =>
                    if b == true {
                        break;
                    },
                Err(e) => panic!("{}",e),
            }
    }

    match mode {
        Mode::Colors => color_mode(),
        Mode::Value => value_mode(),
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
