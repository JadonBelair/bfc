use std::{fs, process::Command, env, time::Instant};

fn main() {

    let mut args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        args.push(String::from(""));
    }

    let file  = match fs::read_to_string(&args[1]) {
        Ok(stuff) => stuff,
        Err(e) => {
            println!("Error: {}", e);
            println!("Usage: bfc filepath [args]");
            return;
        }
    };

    let code = file.chars().collect::<Vec<char>>();

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

    let now = Instant::now();

    let contents = generate_file_text(code);
    
    match fs::write("./".to_owned() + name + ".rs", contents) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {}\nABORTING!", e);
            return;
        }
    }

    Command::new("rustc").args([(name.to_owned() + ".rs").as_str(), "-Clink-arg=/DEBUG:NONE"]).spawn().expect("Failed to compile file");
    let time = now.elapsed();

    if dur {println!("The compiler took {:?}", time)}

    println!("The compiled file can be run with '.\\{}.exe'", name);

}

fn generate_file_text(code: Vec<char>) -> String {
    let mut content = "".to_owned();

    if code.contains(&',') {
        content += "use std::io::{self, Write};\n";
        content += "
fn read(memory: &mut [u8; MEM_SIZE], mem_index: usize) {
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf = buf.trim().to_string();
    let mut input: char = 0 as char;
    if buf.len() != 0 {
        input = buf.chars().collect::<Vec<char>>()[0];
    }
    memory[mem_index] = input as u8
}\n";
    }

    content += "const MEM_SIZE: usize = 30000;\n";

    content += "fn main() {\n";

    if code.contains(&'>') || code.contains(&'<') {
        content += "    let mut mem_index = 0;\n";
    } else {
        content += "    let mem_index = 0;\n";
    }

    content += "    let mut memory: [u8; MEM_SIZE] = [0; MEM_SIZE];\n";

    for c in code.clone() {

        match c {
            // both '<' and '>' have cell rapping features enabled
            '>' => content += "    mem_index = if mem_index == MEM_SIZE - 1 {0} else {mem_index + 1};\n",
            '<' => content += "    mem_index = if mem_index == 0 {MEM_SIZE - 1} else {mem_index - 1};\n",
            // both '+' and '-' wrap the numbers to avoid over/underflow
            '+' => content += "    memory[mem_index] = if memory[mem_index] == 255 {0} else {memory[mem_index] + 1};\n",
            '-' => content += "    memory[mem_index] = if memory[mem_index] == 0 {255} else {memory[mem_index] - 1};\n",
            // prints the current cell's value to the screen in ascii
            '.' => content += "    print!(\"{}\", memory[mem_index] as char);\n",
            // clears stdout stream so that input can be on the same line as a print!()
            ',' => content += "    read(&mut memory, mem_index);\n",
            // will grab the end of the loop from the pre-generated loop table if the current cell's value is zero
            '[' => content += "    while memory[mem_index] != 0 {\n",
            // grabs the start of the current loop using the pre-generated loop table if the current cell's value is not zero
            ']' => content += "    }\n",
            // ignores all other chars
            _ => ()
        }
    }

    content += "}";

    return content;

}