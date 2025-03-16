use std::{
    env,
    fs,
    io::Write
};

#[derive(Default)]
struct Config {
    //whitespace_dep: bool,
    preserve_indent: bool,
    in_path: String,
    out_path: String,
}

fn main() {
    let conf: Config = match set_config() {
        Ok(c) => c,
        Err(e) => panic!("Error in setting config: {}", e)
    };

    let data: Vec<String> = read_contents(&conf);
    let outdata: Vec<String> = right_align_contents(&data, &conf);
    write_lines_to_file(&outdata, &conf.out_path); 
}

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
    }
}

fn right_align_contents(data: &Vec<String>, conf: &Config) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();

    if conf.preserve_indent == false {
        let right_wall = find_right_wall(data, conf);
        for line in data.iter() {
            let diff = right_wall - line.chars().count();
            let newline = format!("{}{}", &" ".repeat(diff), line);
            out.push(newline.clone());
        }
    } 
    else {
        let right_wall = find_right_wall(data, conf);
        for line in data.iter() {
            let indent_level: usize = find_indent_level(&line);
            let diff = right_wall - line.chars().count();
            let newline = format!("{}{}", &" ".repeat(diff - indent_level), line);
            out.push(newline.clone());
        }
    }

    out
}

// #########################################################

fn find_indent_level(line: &str) -> usize {
    let mut indent: usize = 0;
    for char in line.chars() {
        if char == '\t' || char == ' ' {
            indent += 1;
        } else if char != '\t' || char !=' ' {
            return indent
        }
    }
    indent
}

// #########################################################

fn find_right_wall(data: &Vec<String>, conf: &Config) -> usize {
    let mut right_wall: usize = 0;
    if conf.preserve_indent == false {
        for line in data.iter() {
            let len = line.chars().count();
            if len > right_wall {
                right_wall = len;
            }
        }
    }
    else if conf.preserve_indent == true {
        let mut max_indent: usize = 0;
        for line in data.iter() {
            let indent: usize = find_indent_level(line);
            let len = line.chars().count();
            if indent > max_indent {
                max_indent = indent;
            }
            if len > right_wall {
                right_wall = len;
            }
        }
        right_wall += max_indent;
    }
    right_wall
}


// #########################################################

fn read_contents(conf: &Config) -> Vec<String> {
    fs::read_to_string(&conf.in_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

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

fn set_config() -> Result<Config, &'static str> {
    let mut conf = Config::default();
    let args: Vec<String> = env::args().collect();
    let mut arg_iter = args.iter();

    while let Some(arg) = arg_iter.next() {
        if arg.trim() == "--preserve-indent" {
            conf.preserve_indent = true;
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

