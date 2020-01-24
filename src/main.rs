extern crate clap;
use clap::{Arg, App};

extern crate regex;
use regex::Regex;
 
use std::fs;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let matches = App::new("Subtitle Converter")
                          .version("1.0.0")
                          .author("Mohamed Magdi . <migzzawi@gmail.com>")
                          .about("Does awesome things")
                          .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("FILE")
                               .help("Sets a custom config file")
                               .takes_value(true))
                          .arg(Arg::with_name("INPUT")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                         .arg(Arg::with_name("OUTPUT")
                               .help("Sets the output file to use")
                               .required(true)
                               .index(2))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    let input_file_path = matches.value_of("INPUT").unwrap();
    let output_file_path = matches.value_of("OUTPUT").unwrap();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    println!("Using input file: {}", input_file_path);
    println!("Using output file: {}", output_file_path);


    // more program logic goes here...

    let contents = fs::read_to_string(input_file_path).expect("Something went wrong reading this file");

    let vtt_contents = convert_from_srt_to_vtt(&contents);

//     println!("Result is \n{}", vtt_contents);
    write_output(&output_file_path, &vtt_contents);
}


fn convert_from_srt_to_vtt (srt_contents: &String) -> String
{

     let mut vtt_contents = String::from("");

     // add metadata
     vtt_contents.push_str("WEBVTT \n\n");

     let rgx = Regex::new(r"\d").unwrap();
 
     for section in srt_contents.split("\n") {
 
          let is_an_caption_number:bool = rgx.is_match(section) && !section.contains(":");
        
          if !is_an_caption_number
           { 
               vtt_contents.push_str(section);
               vtt_contents.push('\n');
           }
     }


    let rgx = Regex::new(r"\d,\d").unwrap();
    let result = rgx.replace_all(&vtt_contents, ".");

    result.to_string()
}


fn write_output(out: &str, content: &String) {
    let path = Path::new(out);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match fs::File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("{} is ready!", display),
    }
}
