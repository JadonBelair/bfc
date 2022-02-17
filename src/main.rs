use clap::Parser;
use std::{fs, io::{BufReader, BufRead, Write, self}, path::Path, process::Command, time::Instant, collections::HashMap};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of output file (no extension)
    #[clap(short = 'o', default_value = "output")]
    output: String,

    /// Runs the file in interpreted mode
    #[clap(short = 'i')]
    interpret: bool,

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

    let source_path = args.file;

    let source_file = fs::File::open(&source_path).expect("source file doesn't exist");

    if !args.interpret {
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

        generate_file(source_file, file);

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
    } else {
        interpret(source_path);
    }
}

fn generate_file(source: fs::File, mut dest: fs::File) {
    
    let mut reader = BufReader::with_capacity(1, source);

    write!(dest, "use std::io::{{self, Write}};\n").unwrap();
    write!(dest,
"fn read(memory: &mut [u8; MEM_SIZE], mem_index: usize) {{
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
    
    // defines the max size of the memory array
    write!(dest, "const MEM_SIZE: usize = 30000;\n").unwrap();
    write!(dest, "fn main() {{\n").unwrap();
    write!(dest, "    let mut mem_index = 0;\n").unwrap();
    write!(dest, "    let mut memory: [u8; MEM_SIZE] = [0; MEM_SIZE];\n").unwrap();

    while reader.fill_buf().unwrap() != [] {

        let c = reader.fill_buf().unwrap()[0] as char;

        match c {
            // both '<' and '>' have cell rapping features enabled
            '>' => {
                write!(dest, "    mem_index = if mem_index == MEM_SIZE - 1 {{0}} else {{mem_index + 1}};\n").unwrap();
                reader.consume(1);
            },
            '<' => {
                write!(dest, "    mem_index = if mem_index == 0 {{MEM_SIZE - 1}} else {{mem_index - 1}};\n").unwrap();
                reader.consume(1);
            },
            // both '+' and '-' wrap the numbers to avoid over/underflow
            '+' => {
                let mut amount = 1;
                reader.consume(1);

                // calculates the number of "+" 
                // symbols there are in a row
                while reader.fill_buf().unwrap() != [] && reader.fill_buf().unwrap()[0] as char == c {
                    amount += 1;
                    reader.consume(1);
                }
                
                // ensures that the amount to add
                // is always less than 256
                amount %= 256;

                // will only write line if there is a point to doing so
                if amount > 0 {
                    write!(dest, "    memory[mem_index] = ((memory[mem_index] as u16 + {}) % 256) as u8;\n", amount).unwrap();
                }
            },
            '-' => {
                let mut amount: u16 = 1;
                reader.consume(1);

                // calculates the number of "-" 
                // symbols there are in a row
                while reader.fill_buf().unwrap() != [] && reader.fill_buf().unwrap()[0] as char == c {
                    amount += 1;
                    reader.consume(1);
                }

                // ensures that the amount to subtract
                // is always less than 256
                amount %= 256;

                // will only write line if there is a point to doing so
                if amount > 0 {
                    write!(dest, "    memory[mem_index] = ((memory[mem_index] as i16 - {}) % 256) as u8;\n", amount).unwrap();
                }
            },
            // prints the current cell's value to the screen in ascii
            '.' => {
                write!(dest, "    print!(\"{{}}\", memory[mem_index] as char);\n").unwrap();
                reader.consume(1);
            },
            // clears stdout stream so that input can be on the same line as a print!()
            ',' => {
                write!(dest, "    read(&mut memory, mem_index);\n").unwrap();
                reader.consume(1);
            },
            // will grab the end of the loop from the pre-generated loop table if the current cell's value is zero
            '[' => {
                write!(dest, "    while memory[mem_index] != 0 {{\n").unwrap();
                reader.consume(1);
            },
            // grabs the start of the current loop using the pre-generated loop table if the current cell's value is not zero
            ']' => {
                write!(dest, "    }}\n").unwrap();
                reader.consume(1);
            },
            // ignores all other chars
            _ => reader.consume(1)
        }

    }

    // closes the main function
    write!(dest, "}}").unwrap();
}

fn interpret(source: String) {

    const MEM_SIZE: usize = 30000;

    let content = fs::read_to_string(source);

    let code = match content {
        Ok(stuff) => {stuff.chars().collect::<Vec<char>>()},
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let mut mem_index = 0;
    let mut instruction_pointer = 0;

    let mut memory: [u8; MEM_SIZE] = [0; MEM_SIZE];

    let loop_table = generate_loop_table(&code);

    let code_length = code.len();

    while instruction_pointer < code_length {
        let c = code[instruction_pointer];

        match c {
            // both '<' and '>' have cell rapping features enabled
            '>' => mem_index = if mem_index == MEM_SIZE - 1 {0} else {mem_index + 1},
            '<' => mem_index = if mem_index == 0 {MEM_SIZE - 1} else {mem_index - 1},

            // both '+' and '-' wrap the numbers to avoid over/underflow
            '+' => memory[mem_index] = if memory[mem_index] == 255 {0} else {memory[mem_index] + 1},
            '-' => memory[mem_index] = if memory[mem_index] == 0 {255} else {memory[mem_index] - 1},
            // prints the current cell's value to the screen in ascii
            '.' => print!("{}", memory[mem_index] as char),
            // clears stdout stream so that input can be on the same line as a print!()
            ',' => {
                io::stdout().flush().unwrap();
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).unwrap();
                buf = buf.trim().to_string();
                let mut input: char = 0 as char;
                if buf.len() != 0 {
                    input = buf.chars().collect::<Vec<char>>()[0];
                }
                memory[mem_index] = input as u8;
            },
            // will grab the end of the loop from the pre-generated loop table if the current cell's value is zero
            '[' => {
                if memory[mem_index] == 0 {

                    instruction_pointer = loop_table.get(&instruction_pointer).unwrap().clone();
                }
            },
            // grabs the start of the current loop using the pre-generated loop table if the current cell's value is not zero
            ']' => {
                if memory[mem_index] != 0 {
                    instruction_pointer = loop_table.get(&instruction_pointer).unwrap().clone();
                }
            },
            // ignores all other chars
            _ => ()
        }

        instruction_pointer += 1;
    }
}

// generates a hashmap of all the loops and their starting/ending points
fn generate_loop_table(code: &Vec<char>) -> HashMap<usize, usize> {

    let mut loop_stack: Vec<usize> = Vec::new();
    let mut loop_table: HashMap<usize, usize> = HashMap::new();

    for (ip, instruction) in code.iter().enumerate() {
        match *instruction {
            '[' => {
                loop_stack.push(ip);
            },
            ']' => {
                let loop_start = loop_stack.pop().unwrap();
                loop_table.insert(loop_start, ip);
                loop_table.insert(ip, loop_start);
            },
            _ => ()
        }
    }

    loop_table
}