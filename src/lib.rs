use std::fs;
use std::error::Error;
use std::env;
pub struct Config{
    pub file_name: String,
    pub query: String,
    pub case_insensitive: bool,
}

impl Config{
    pub fn new(mut args:std::env::Args) ->Result<Config,&'static str>{
        args.next();
        let query = match args.next(){
            Some(t)=>t,
            None=> return Err("not found query"),
        };
        let file_name = match args.next() {
            Some(t)=>t,
            None=> return Err("not found file name"),
        };
        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            file_name,
            case_insensitive,
        })
    }
}

pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_name)?;
    let results = if config.case_insensitive{
        search_case_insensitive(&config.query,&contents)
    }else{
        search(&config.query,&contents)
    };
    for line in  results{
        println!("{}",line);
    }
    Ok(())
}

pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}
pub fn search_case_insensitive<'a>(query:&str,contents:&'a str)-> Vec<&'a str>{
    contents.lines()
        .filter(|line| line.to_lowercase().as_str().contains(query.to_lowercase().as_str()))
        .collect()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query  = "uct";
        let contents = "\
        product,fast,reliable
        friend
        ";
        assert_eq!(search_case_insensitive(query,contents),vec!["product,fast,reliable"]);
    }
    #[test]
    fn  case_insensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["Rust:","Truest me."],search(query,contents));
    }
}
