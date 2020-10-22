fn main() {
    let mut the_ints = vec![1, 2, 3, 4, 5];
    for int in the_ints.iter(){
        println!("the int {}",int);
    }
    for int in  the_ints.iter_mut(){
        // 注意 * 用来解引用，就是直接修改被引用的值
        *int = *int+12;
    }
    for int in  the_ints.iter_mut(){
        // 注意 * 用来解引用，就是直接修改被引用的值
        println!("after inter_mut:{}",int);
    }
    let the_strings = vec![1.to_string(),2.to_string(),3.to_string()];
    for string in the_strings.into_iter(){
        println!("the string:{}",string);
    }
    // 在这里 the_strings 已经 moved 而不能使用了
    // println!("the_strings:{:?}",the_strings);
}