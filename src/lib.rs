use std::{
    error::Error,
    fs::File,
    io::{self, Read},
};

pub struct Config {
    pub filename: String,
    pub get_num_of_bytes: bool,
    pub get_num_of_lines: bool,
    pub get_num_of_words: bool,
    pub get_num_of_chars: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        let mut config = Config {
            filename: String::new(),
            get_num_of_bytes: false,
            get_num_of_chars: false,
            get_num_of_lines: false,
            get_num_of_words: false,
        };

        for arg in args.iter().skip(1) {
            if arg == "-c" {
                config.get_num_of_bytes = true;
            } else if arg == "-l" {
                config.get_num_of_lines = true;
            } else if arg == "-w" {
                config.get_num_of_words = true;
            } else if arg == "-m" {
                config.get_num_of_chars = true;
            } else if arg.starts_with("-") {
                let msg = format!("invalid argument {} provided", arg);
                return Err(msg);
            } else {
                config.filename = arg.clone();
            }
        }

        if args.len() == 1 || (args.len() == 2 && !config.filename.is_empty()) {
            config.get_num_of_bytes = true;
            config.get_num_of_lines = true;
            config.get_num_of_words = true;
        }

        Ok(config)
    }

    fn get_final_result(self: &Config, contents: &str, results: &mut Vec<usize>) {
        if self.get_num_of_lines {
            results.push(get_num_of_lines(&contents));
        }
        if self.get_num_of_words {
            results.push(get_num_of_words(&contents));
        }
        if self.get_num_of_bytes {
            results.push(get_num_of_bytes(&contents));
        }
        if self.get_num_of_chars {
            results.push(get_num_of_chars(&contents));
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();

    if config.filename.is_empty() {
        read_from_stdin(&mut contents)?;
    } else {
        let mut f = File::open(&config.filename)?;
        f.read_to_string(&mut contents)?;
    }

    let mut results: Vec<usize> = Vec::new();

    config.get_final_result(&contents, &mut results);

    let final_result = results
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>()
        .join("   ");

    println!("   {}   {}", final_result, config.filename);

    Ok(())
}

fn get_num_of_bytes(contents: &str) -> usize {
    contents.len()
}

fn get_num_of_lines(contents: &str) -> usize {
    contents.lines().count()
}

fn get_num_of_words(contents: &str) -> usize {
    let mut count: usize = 0;

    for line in contents.lines() {
        count += line.split_whitespace().count();
    }

    count
}

fn get_num_of_chars(contents: &str) -> usize {
    contents.chars().count()
}

fn read_from_stdin(contents: &mut String) -> Result<(), String> {
    match io::stdin().read_to_string(contents) {
        Ok(_) => Ok(()),
        Err(_) => Err("could not read from standard input".to_string()),
    }
}
