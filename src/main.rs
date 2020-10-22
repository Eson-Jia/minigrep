use std::env;
use std::process;
use minigrep::{Config};

fn add_one(origin:u32)->u32{
      origin+1
}


#[cfg(test)]
mod tests{
    use crate::add_one;

    #[test]
    fn test_add_one(){
        assert_eq!(add_one(100),101);
    }
}


fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}",err);
        process::exit(1);
    });
    if let Err(err) = minigrep::run(config){
        eprintln!("Applications error: {}",err);
        process::exit(1);
    };
}
