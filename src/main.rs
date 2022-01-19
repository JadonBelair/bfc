use std::{fs, io::Write, process::Command, env, time::Instant, path::Path};

fn main() {

    let mut args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        args.push(String::from(""));
    }

    let code: Vec<char>  = match fs::read_to_string(&args[1]) {
        Ok(stuff) => stuff.chars().collect(),
        Err(e) => {
            println!("Error: {}", e);
            println!("Usage: bfc [filepath] [flags]");
            return;
        }
    };

    let flags = &args[2..];

    let mut dur = false;


    let mut name = "output";

    for (i, f) in flags.iter().enumerate() {
        match f.as_str() {
            "-time" => dur = true,
            "-o" => name = flags[i+1].as_str(),
            _ => ()
        }
    }

    // gets path to the outputted rust file
    let path = "./".to_owned() + name + ".rs";

    // if the rust file exists, it gets deleted
    let p = Path::new(path.as_str());
    if p.exists() {
        fs::remove_file(p).expect("error initializing file");
        // this just makes sure that the old file is deleted before continuing
        while p.exists() {

        }
    }

    // creates a new rust file at the specified path
    let file = fs::OpenOptions::new().append(true).create(true).open(p).expect("error creating file");

    let now = Instant::now();

    generate_file(code, file);
    
    Command::new("rustc").args([(name.to_owned() + ".rs").as_str(), "-Clink-arg=/DEBUG:NONE"]).spawn().expect("Failed to compile file");
    let time = now.elapsed();

    if dur {println!("The compiler took {:?}", time)}

    println!("The compiled file can be run with '.\\{}.exe'", name);

}

fn generate_file(code: Vec<char>, mut file: std::fs::File) {
    // will add the needed crate and method to read user input
    // only if the user input command is found in source code
    if code.contains(&',') {
        write!(file, "use std::io::{{self, Write}};\n").unwrap();
        write!(file, "
fn read(memory: &mut [u8; MEM_SIZE], mem_index: usize) {{
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf = buf.trim().to_string();
    let mut input: char = 0 as char;
    if buf.len() != 0 {{
        input = buf.chars().collect::<Vec<char>>()[0];
    }}
    memory[mem_index] = input as u8
}}\n").unwrap();
    }

    // defines the max size of the memory array
    write!(file, "const MEM_SIZE: usize = 30000;\n").unwrap();

    write!(file, "fn main() {{\n").unwrap();

    // will only bother making the memory index mutable if needed
    if code.contains(&'>') || code.contains(&'<') {
        write!(file, "    let mut mem_index = 0;\n").unwrap();
    } else {
        write!(file, "    let mem_index = 0;\n").unwrap();
    }

    write!(file, "    let mut memory: [u8; MEM_SIZE] = [0; MEM_SIZE];\n").unwrap();

    for c in code {

        match c {
            // both '<' and '>' have cell rapping features enabled
            '>' => write!(file, "    mem_index = if mem_index == MEM_SIZE - 1 {{0}} else {{mem_index + 1}};\n").unwrap(),
            '<' => write!(file, "    mem_index = if mem_index == 0 {{MEM_SIZE - 1}} else {{mem_index - 1}};\n").unwrap(),
            // both '+' and '-' wrap the numbers to avoid over/underflow
            '+' => write!(file, "    memory[mem_index] = if memory[mem_index] == 255 {{0}} else {{memory[mem_index] + 1}};\n").unwrap(),
            '-' => write!(file, "    memory[mem_index] = if memory[mem_index] == 0 {{255}} else {{memory[mem_index] - 1}};\n").unwrap(),
            // prints the current cell's value to the screen in ascii
            '.' => write!(file, "    print!(\"{{}}\", memory[mem_index] as char);\n").unwrap(),
            // clears stdout stream so that input can be on the same line as a print!()
            ',' => write!(file, "    read(&mut memory, mem_index);\n").unwrap(),
            // will grab the end of the loop from the pre-generated loop table if the current cell's value is zero
            '[' => write!(file, "    while memory[mem_index] != 0 {{\n").unwrap(),
            // grabs the start of the current loop using the pre-generated loop table if the current cell's value is not zero
            ']' => write!(file, "    }}\n").unwrap(),
            // ignores all other chars
            _ => ()
        }
    }

    // closes the main function
    write!(file, "}}").unwrap();
}