use std::fs;
use std::error::Error;
use std::assert_eq;
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

pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    let mut result = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            result.push(line);
        }
    }
    result
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
        assert_eq!(search(query,contents),vec!["product,fast,reliable"]);
    }
}
