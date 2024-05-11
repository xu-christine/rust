//变量绑定与解构
fn varible()
{
    //变量的绑定与解构
    let x = 1;  //绑定
    let _y = 2; //下划线开头的变量，忽略未使用，不提示警告
    let mut z=3; //变量值可以修改，所有权可以转移，变量可变性

    //变量解构
    let(a,mut b):(bool,bool)=(true,false);
    b=true;

    //变量与常量
    let m = 4;
    const _MAX_VALUE: i32=100_00; //常量用const修饰，且不能使用mut关键字 

    //变量遮蔽，使用{}
    let n= 5;
    {
        let n= 6;
        println!("{}", n);
    }
    println!("{}", n);

    //所有未使用的变量，会给出警告：unused variable...
}

//所有权和借用
fn ownership()
{
    //所有权:变量所赋予的值，仅在{}范围内生效
    //栈上的空间消耗较小，且不用自动释放 
    let a=2; 
    let b: i32=a; 

    //浅拷贝
    let x="hello rust";
    let y=x; //"hello rust"的所有权转移到y
    //深拷贝
    let s1="hello";
    let s2=s1.clone();

    //引用与借用
    let m= 5;
    let n= &m;

    //借用规则：1.引用必须有效
    //        2.同一时刻，只能拥有一个可变引用或者多个不可变饮用
    let p= 5;
    let q= &p;
    let mut s= 6;
    let r: &mut i32= &mut s;
}
