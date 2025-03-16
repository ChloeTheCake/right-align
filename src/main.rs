use std::{
    io,
    env,
    fs,
    io::Write
};

#[derive(Default)]
struct Config {
    indent_dep: bool,
    in_path: String,
    out_path: String,
}

fn main() {
    sanity_check();
    let conf: Config = match set_config() {
        Ok(c) => c,
        Err(e) => panic!("Error in setting config: {}", e)
    };

    let data: Vec<String> = read_contents(&conf);
    let outdata: Vec<String> = right_align_contents(&data, &conf);
    write_lines_to_file(&outdata, &conf.out_path); 
}

// #########################################################
// #########################################################
// #########################################################


fn write_lines_to_file(outdata: &Vec<String>, name: &str) {
    if !is_valid_file(&name) {
        println!("Attempting to create the file: {}", name);
        match fs::File::create(&name) {
            Ok(_) => (),
            Err(e) => panic!("Cannot create non-existent file\nError: {}", e)
        }
    }

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(name)
        .expect("Error opening out file");
    for line in outdata.iter() {
        file.write(line.as_bytes()).expect("failure to write to output");
        file.write("\n".as_bytes()).expect("failure to write to output");



        //match fs::write(name, line). {
        //    Ok(_) => (),
        //    Err(e) => panic!("Failure to write to output\nError: {}", e)
        //}
    }
}

fn right_align_contents(data: &Vec<String>, _conf: &Config) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let right_wall = find_right_wall(data);
    for line in data.iter() {
        let len = line.chars().count();
        let diff = right_wall - len;
        let newline = format!("{}{}", &" ".repeat(diff), line);
        out.push(newline.clone());


    }
    out
}

// #########################################################
// #########################################################
// #########################################################

fn find_right_wall(data: &Vec<String>) -> usize {
    let mut right_wall: usize = 0;
    for line in data.iter() {
        let len = line.chars().count();
        if len > right_wall {
            right_wall = len;
        }
    }
    right_wall
}


// #########################################################
// #########################################################
// #########################################################

fn read_contents(conf: &Config) -> Vec<String> {
    fs::read_to_string(&conf.in_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

// #########################################################
// #########################################################
// #########################################################

fn is_valid_file(path: &str) -> bool {
    match fs::File::open(path) {
        Ok(_) => true,
        Err(e) => {
            println!("{} does not exist: {}", path, e);
            false
        }
    }

}

// #########################################################
// #########################################################
// #########################################################

fn set_config() -> Result<Config, &'static str> {
    let mut conf = Config::default();
    let args: Vec<String> = env::args().collect();
    let mut arg_iter = args.iter();

    while let Some(arg) = arg_iter.next() {
        if arg.trim() == "--indent-dep" {
            conf.indent_dep = true;
        }

        if arg.trim() == "--input" {
            if let Some(path) = arg_iter.next() {
                if is_valid_file(path) {
                    conf.in_path = path.to_string();
                }
                else {
                    return Err("Not a valid file");
                }
            }
            else {
                return Err("No path for --input was specified");
            }
        }

        if arg.trim() == "--output" {
            if let Some(name) = arg_iter.next() {
                conf.out_path = name.to_string();
            }
            else {
                return Err("No output file name specified");
            }
        }
    }

    Ok(conf)
}

// #########################################################
// #########################################################
// #########################################################

fn sanity_check() {
    println!("You're about to commit a sin, wanna continue? (y/n): ");
    let mut p = String::new();
    match io::stdin().read_line(&mut p) {
        Ok(_n) => { 
            if p.trim() != "y" {
                println!("\nOk\n");
                return;
            }
        }
        Err(e) => panic!("Error in sanity_check: {}", e),
    };
}
