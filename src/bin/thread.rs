#[cfg(test)]
mod tests{
    use std::thread;
    use std::time::Duration;
    #[test]
    fn test_tread_spawn(){
        let  child = thread::spawn(||{
            for i in 0..10{
                println!("child thread sleep for 1 second:{}",i);
                thread::sleep(Duration::from_secs(1));
            }
        });
        for i in 0..6 {
            println!("main thread sleep for 1 second:{}",i);
            thread::sleep(Duration::from_secs(1));
        }
        child.join().expect("join failed");
    }
    #[test]
    fn test_thread_move(){
        let info =String::from("this is a information");
        thread::spawn(move||{
            thread::sleep(Duration::from_secs(1));
            println!("{}",info.to_uppercase());
        }).join();
        println!("the info can't use here");
    }
}



fn main() {}