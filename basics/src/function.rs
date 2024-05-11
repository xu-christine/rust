use std::fmt::Display;
use std::ops::Sub;

pub fn sub<T: Sub<Output = T>>(a: T, b: T) -> T {
    a - b
}

//函数
pub fn add(a: i32 ,b: i32) -> i32 //函数类型 函数名 (参数1: 类型， 参数2: 类型) -> 返回值类型
{
    a+b //返回值
}

pub fn printf<T: Display>(value: T) {
    println!("value: {}", value);
}
