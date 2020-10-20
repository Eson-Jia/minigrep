use std::fs;
use std::error::Error;
use std::env;
pub struct Config{
    pub file_name: String,
    pub query: String,
    pub case_insensitive: bool,
}

impl Config{
    pub fn new(args:&[String]) ->Result<Config,&'static str>{
        if args.len()<3{
            return Err("wrong arguments numbers");
        }
        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query: args[1].clone(),
            file_name: args[2].clone(),
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
    let mut result = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            result.push(line);
        }
    }
    result
}
pub fn search_case_insensitive<'a>(query:&str,contents:&'a str)-> Vec<&'a str>{
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
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
