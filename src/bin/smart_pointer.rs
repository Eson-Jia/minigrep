

#[cfg(test)]
mod tests{
    #[test]
    fn test_ref(){
        //可以将引用看做一种指针
        // 如果一个引用想要修改被引用变量的值就需要以下步骤
        // 将被修改的变量声明为可变的
        let mut a = 123;
        // 创建一个可变引用
        let r_a = &mut a;
        // 使用星号解引用来修改引用所指向的变量的值
        *r_a+=1;
        println!("a:{}",a);
    }
    #[test]
    fn test_pointer(){
        let mut b = Box::new(111);
        *b+=1;
        println!("b:{}",b);
    }
}

fn main() {

}