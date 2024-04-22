//流程控制
/*fn main() {
    //ifxx{} else ifxx{} else{}
    //eg:
    let x=10;
    if x%5 == 0{
        println!("num can be devisable by 5");
    }else if x%3 == 0{
        println!("num can be devisable by 3");
    }else{
        println!("num can not devisable by 5 or 3");
    }

    //循环遍历：for/while
    //关键字:
    //continue: 跳出本次循环，执行下一次循环
    //break: 直接跳出循环

    for i in 0..10{
        if i == 5{
            continue;
        }
        if i == 9{
            break;
        }
        println!("{}", i);
    }

    let mut idx = 0;
    while idx <= 10{
        if idx == 6{
            continue;
        }
        if idx == 8{
            break;
        }

        println!("{}", idx);
        idx+=1;
    }
}
*/

enum Body {
    Head,
    Eye,
    Hand,
    Foot,
}
//模式匹配
/*fn main() {
    //match: 类似switch，穷举出所有可能
    let mat=Body::Eye;
    match mat{
        Body::Eye=> println!("eye!"),
        Body::Hand|Body::Head =>println!("Head or Hand!"),
        _ => println!("Foot!"), //类似switch中的default, _ 变量:通配符
    }

    //if let: 单条件匹配，不用穷举，只匹配需要的情况,其他条件忽略
    let num=Some(5);
    if let Some(5) = num{
        println!("success !");
    }

    //Option: 两种类型：有值Some(T)/空 None
    enum Option<T>
    {
        Some(T),
        None,
    }

    //总结
    //全匹配模式: match, for, while
    //部分匹配模式: if let, while let
}*/

//方法Method
//rust的方法类似c++的成员函数, 但是方法并不在类内，提供了极高的灵活性，有点C像语言结构，支持类结构调用
fn main() {}
