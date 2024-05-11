//类型
fn data_tpye()
{
    //数值类型
      //整形  ：i8, i16, i32, i64, isize
      //无符号：u8, u16, u32, u64, usize
    let a: i8= 1;
    let b: i16= 2;
    let c: i32= 3; //rust默认i32
    
    //浮点型
      //单精度浮点型：f32, f64
      //双精度浮点型：d32, d64
    let d: f32= 1.00;
    let e: f64= 2.00;
    
    //字符类型
     //字符： char
     //字符串：string
    let h: char= 'c';
    let i:String= String::from("hello");
    let strs:&str= "hello rust";

    //布尔类型：true/false
    let j: bool= true;
    
    //单元类型：()
    let k=(); //作为返回值使用
    //As显式类型转换
    let l: i32 = d as i32;

    //序列 :1..=5
    for i in 1..=5{
        println!("{}", i);
    }
}

//复合类型
//字符串
fn compsition_type() {
    //字符串: 默认类型是&str
    let s1="hello";
    //切片
    //注: 切片必须关注字符串类型，rust以unicode类型为标准，注意不同类型所占字节数不同
    //英文: 1byte
    //中文: 3byte
    let s1_03= &s1[..3];
    let s1_tostring= s1.to_string();//str->String

    //String类型
    let s2: String=String::from("hello world");
    let s2_str= s2.as_str(); //String->str

    //操作String
    //前提: mut :可变字符串变量
    let mut s3=String::from("hello");
    //push(ch): 追加字符
    s3.push(',');
    //push_str(String): 追加字符串
    s3.push_str("rust");
    //+=:追加字符串
    s3 += "world";

    //insert(pos, ch): 插入
    s3.insert(11,'!');
    
    //replace(from,to): 提换
    let s3_replace= s3.replace("rust", "RUST");
    //replacen(parm1,pram2, count): 有替换个数的替换(第三个参数决定替换个数)
    let s3_replacen= s3.replacen("!"," ",1);


    //delete: 删除
    //pop():删除并返回字符串的最后一个字符
    let s3_pop= s3.pop();
    //remove(pos):删除字符串并返回字符串中指定位置的字符
    let s3_remove=s3.remove(5);
    //truncate(start):
    s3.truncate(7);
    //clear:  清空
    s3.clear();
}

//元组: 使用场景不多
fn tuple()
{
    //多种类型组合在一起的数据结构，具有长度限制
    let tup=(500,2.00,'c');
    
    //解构
    let x=tup.0;
    let y=tup.1;
    let z=tup.2;
}

//结构体
pub struct User{
    id: i32,
    name: String,
    email: String,
    active: bool,
}

//rust struct可以类比c++中的类
//impl方法： 类似c++成员函数
impl User {
    fn new(user: User) -> Self {
        let User { id, name, email, active } = user;
        Self { id, name, email, active }
    }
}

//枚举
enum Env
{
    Types,
    Id
}

//数组:类型一致，连续的空间，适合查询
//定长数组 array
//动态数组 vector
fn fixd_size_array()
{

    //定长数组: 根据下标驱动
    let array=[1,2,3,4,5];
    
    //动态数组和c++STL中vector类似
    //注: 修改必加 mut
    
    let mut v=Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    //判空
    v.is_empty();
    //插入
    v.insert(4,6);
    v.push(7);
    //遍历
    for i in &v
    {
        println!("{}", *i);
    }
    //删除
    v.remove(0);

    //排序:
      //稳定: sort, sort_by
    v.sort();
      //非稳定: sort_unstable, sort_unstable_by
    //清空
    v.clear();
}