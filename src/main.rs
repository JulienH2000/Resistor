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
    Exit
}

fn menu_check (i:&str) -> Result<Mode, &'static str> {
    let buff: Vec<&str> = i.split_whitespace().collect();
    match buff[0].to_lowercase().as_str() {
        "colors" | "color" => return Ok(Mode::Colors),
        "values" | "value" => return Ok(Mode::Value),
        _ => Err("Mode invalid !!"),
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

        println!("{}",calc_resistor(resistor.set_mult_for_colors().unwrap().set_tolerance_for_colors().unwrap()));
    }
}

fn value_mode() {
    println!("entering value mode...\n");
    /*'tolerance: loop {
        println!("Enable tolerance input ?\n");
        let user_input = get_user_input();
        break;
    }*/
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

        let values;
        match string_to_values(user_input) {
            Ok(r) => values = r,
            Err(e) => {println!("{}",e);
            continue;
            }
        }
        let colors = values.to_colors().unwrap();
        println!("{} {} {} {}", colors.first_color.unwrap(), 
                                colors.sec_color.unwrap(), 
                                colors.third_color.unwrap_or(String::from("")), 
                                colors.mult.unwrap());

    
    }
}

fn main () {
    banner();
    println!("This software converts your resistor color code in Ohms,\nUse english colors, with spaces. This is case insensitive.\nType \"exit\" to quit");

    let mode: Mode;

    mode = loop {
        println!("Select input mode: Colors / Value, or exit");
        let user_input = get_user_input();
            match menu_check(&user_input) {
                Ok(m) => break m ,
                Err(e )=> println!("{}", e),
            }
            match exit_check(&user_input) {
                Ok(b) =>
                    if b == true {
                        break Mode::Exit;
                    },
                Err(e) => panic!("{}",e),
            }
    };

    match mode {
        Mode::Colors => color_mode(),
        Mode::Value => value_mode(),
        Mode::Exit => (),
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
