#[derive(Debug)]
enum IpAddrs{
    V4 ,
    V6 , 
}
fn main(){
    let home: IpAddrs = IpAddrs::V4 ;
    let loopback : IpAddrs = IpAddrs::V6 ;
    println!("Home is :{:?}" , home) ;
    println!("Loopback is :{:?}" , loopback) ;
}
_____________________________________________________________________________________________________________________________________
#[derive(Debug)]
enum IpAddr{
    V4(String) ,
    V6(String) , 
}
fn main(){
    let home : IpAddr = IpAddr::V4(String::from("192.168.1.101")) ;
    let loopback : IpAddr = IpAddr::V6(String::from("::1")) ;
    match home{
        IpAddr::V4(val_1) => {
            println!("Val 1 is -> :{:?}" , val_1) ; 
        }
        IpAddr::V6(val_2) =>{
            println!("Val 2 is -> :{:?}" , val_2) ; 
        }
    }
    match loopback{
        IpAddr::V4(val_1) =>{
            println!("val 1 is :{:?}" , val_1) ;
        }
        IpAddr::V6(val_2) =>{
            println!("Val 2 is :{:?}", val_2)  ; 
        }
    }
}
___________________________________________________________________________________________________________________________________
#[derive(Debug)]
enum Value{
    Type1(String) ,
    Type2(String) , 
}
#[derive(Debug)]
struct IpInfo{
    name : String ,
    value : Value , 
}
impl IpInfo{
    fn print_ip_info(ip_info : &IpInfo){
        match &ip_info.value{
            Value::Type1(val_1) =>{
                println!("Name is :{:?} and address value is :{:?}" , ip_info.name , val_1) ; 
            }
            Value::Type2(val_2) =>{
                println!("Name is :{:?} and address value is :{:?}" , ip_info.name , val_2) ; 
            }
        }
    }
}
fn main(){
    let ip_info_1 : IpInfo = IpInfo {
        name : String::from("Home") ,
        value : Value::Type1(String::from("192.168.1.101")) , 
    } ;
    let ip_info_2 : IpInfo = IpInfo{
        name : String::from("Loopback") ,
        value : Value::Type2(String::from("::1")), 
    } ;
    IpInfo::print_ip_info(&ip_info_1) ;
    IpInfo::print_ip_info(&ip_info_2) ;  
}
__________________________________________________________________________________________________________________________

