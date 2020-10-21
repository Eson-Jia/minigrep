

//两个 closure即使拥有完全相同函数签名，也会被认为是两种不同的类型，所以要使用模板和trait
struct Cache<T:Fn(u32)->u32> {
    calculation:T,
    element:Option<u32>,
}

impl<T> Cache<T>
where T:Fn(u32)->u32
{
    pub fn new(calculation:T)->Cache<T>{
        Cache{
            calculation,
            element:None,
        }
    }
    pub fn value(&mut self, arg:u32) ->u32{
        match self.element {
            Some(t)=>t,
            None =>{
                let the_value = (self.calculation)(arg);
                self.element = Some(the_value);
                the_value.clone()
            },
        }
    }
}




fn main() {
    let origin =String::from("19");
    let mut cache = Cache::new(/*move*/ |input| input +origin.parse::<u32>().unwrap());
    let  result = cache.value(1);
    println!("result:{},origin:{}",result,origin);
    let result = cache.value(2);
    println!("result:{}",result);
}