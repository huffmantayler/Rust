use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};


type MyResult<T> = Result<T, Box<dyn Error>>;


pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file)
        .map_err(|e| format!("{}: {}", config.in_file, e))?;
        let mut line = String::new();
        let mut line2 = String::new();
        let mut line3 = String::new();
        let mut count = 1;
        let mut counts = HashMap::new();
        loop{
            let bytes = file.read_line(&mut line)?;
            if bytes == 0 {
                break;
            }
            counts.insert()
            if line2 == line {
                count += 1;
            } else {
                print!("{} {}", count, line);
                count = 0;
            }
            
            line2 = line.clone();
            line3 = line2.clone();
            line.clear();
        }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}


pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniqr")
        .version("0.1.0")
        .author("Taylor Huffman <thuffman14@radford.edu")
        .about("Rust uniq")
        .arg(
            Arg::with_name("in_file")
                .value_name("IN_FILE")
                .help("Input file")
                .default_value("-")
        )
        .arg(
            Arg::with_name("out_file")
                .value_name("OUT_FILE")
                .help("Output file")
        )
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .help("prefix lines by the number  of occurences")
        )
        .get_matches();

        Ok(Config {
            in_file: matches.value_of_lossy("in_file").unwrap().to_string(),
            out_file: matches.value_of("out_file").map(String::from),
            count: matches.is_present("count"),
        })
}