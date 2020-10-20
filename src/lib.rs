use std::fs;
use std::error::Error;

pub struct Config{
    pub file_name: String,
    pub query: String,
}

impl Config{
    pub fn new(args:&[String]) ->Result<Config,&'static str>{
        if args.len()<3{
            return Err("wrong arguments numbers");
        }
        Ok(Config {
            file_name: args[1].clone(),
            query: args[2].clone(),
        })
    }
}

pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_name)?;
    println!("contents:{}",contents);
    Ok(())
}