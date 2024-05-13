//引入模块，即对应的文件
mod function;
mod types;
mod variable;

//引入对应函数
use crate::function::{add, sub, printf};

fn main()
{
    let n1=1;
    let n2=3;
    let sum = add(n1,n2);

    printf(sum);
}