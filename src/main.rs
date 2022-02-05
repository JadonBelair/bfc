use clap::Parser;
use std::{fs, io::Write, path::Path, process::Command, time::Instant};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of output file (no extension)
    #[clap(short = 'o', default_value = "output")]
    output: String,

    /// Shows how long compiler took
    #[clap(short = 't', long)]
    time: bool,

    /// Will prevent the compiler from generating output to console
    #[clap(short = 'q', long)]
    quiet: bool,

    /// Will run rustfmt on the generated rust file
    #[clap(short = 'p', long)]
    pretty: bool,

    /// File to compile
    #[clap(name = "FILE")]
    file: String,
}

fn main() {
    let args = Args::parse();

    let source_file = args.file;

    let code: Vec<char> = match fs::read_to_string(&source_file) {
        Ok(stuff) => stuff.chars().collect(),
        Err(e) => {
            println!("Error: {}", e);
            println!("run bfc --help for help");
            return;
        }
    };

    let name = args.output;

    // gets path to the outputted rust file
    let mut path = Path::new(".").join(name);
    path.set_extension("rs");

    // if the rust file exists, it gets deleted
    if path.exists() {
        fs::remove_file(&path).expect("error initializing file");
        // this just makes sure that the old file is deleted before continuing
        while path.exists() {}
    }

    // creates a new rust file at the specified path
    let file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&path)
        .expect("error creating file");

    let now = Instant::now();

    generate_file(code, file);

    // formats the generated rust file if the pretty flag was set
    if args.pretty {
        Command::new("rustfmt")
            .args([path.to_str().unwrap()])
            .spawn()
            .expect("Failed to compile file");
    }

    // compiles the generated rust file using rustc
    Command::new("rustc")
        .args([path.to_str().unwrap(), "-Clink-arg=/DEBUG:NONE"])
        .output()
        .expect("Failed to compile file");

    let time = now.elapsed();

    // will tell the user how long compilation took if the time flag was set
    if args.time && !args.quiet {
        println!("The compiler took {:?}", time)
    }

    path.set_extension("exe");
    if !args.quiet {
        println!("The compiled file can be run at '{}'", path.to_str().unwrap());
    }
}

fn generate_file(code: Vec<char>, mut file: fs::File) {
    // will add the needed crate and method to read user input
    // only if the user input command is found in source code
    if code.contains(&',') {
        write!(file, "use std::io::{{self, Write}};\n").unwrap();
        write!(file,
            "
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
