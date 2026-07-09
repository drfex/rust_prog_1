#[derive(Debug)]
struct Info{

    name : String , 
    age : i32 , 
}
fn  main(){

    let info_1 : Info = Info{

        name : String::from("Kohee") , 
        age :  32 as i32 , 
    } ; 
    println!("Info1-> name is :{:?} and age is {:?} "  , info_1.name , info_1.age) ; 
}
_____________________________________________________________________________________________________________________________
#[derive(Debug)]
struct Info{
    name : String ,
    username : String ,
    age : i32 ,
}
fn main(){

    let mut info_1 : Info = Info{

        name : String::from("Kohee") ,
        username : String::from("Kiaff") ,
        age : 32 as i32 ,
    } ;
    info_1.name = String::from("Kiaff") ;
    println!("Info 1 -> name is :{:?} username is :{:?} and  age is :{:?}" , info_1.name , info_1.username , info_1.age) ;
}
_____________________________________________________________________________________________________________________________
#[derive(Debug)]
struct Info{
    name : String  ,
    age : i32 ,
}
impl  Info{

    fn build_user_1(name : String) -> Info{

        return Info{

            name : name ,
            age : 31 as i32 ,
        } ;
    }
}
fn build_user_2(name : String) -> Info{

    return Info{

        name : name ,
        age : 32 as i32 ,
    } ;
}
fn main(){
    let info_1 : Info = Info::build_user_1(String::from("Kiaff")) ;
    let info_2 : Info = build_user_2(String::from("Sohee")) ;
    println!("Info1 -> name is :{:?} and age is :{:?}" , info_1.name , info_1.age) ;
    println!("Info2 -> name is :{:?} and age is :{:?}" , info_2.name , info_2.age) ;
}
______________________________________________________________________________________________________________________________
#[derive(Debug)]
struct Color1(i32 , i32 , i32) ;
#[derive(Debug)]
struct Color2(i32 , i32 , i32) ; 
fn main(){

    let color_1 : Color1 = Color1(10 , 11 , 12) ;
    let color_2 : Color2 = Color2(20 , 21 , 22) ; 
    println!("Color 1 -> :{:?}:{:?}:{:?}" ,color_1.0 , color_1.1 ,  color_1.2)  ; 
    println!("Color 2 -> :{:?}:{:?}:{:?}" ,color_2.0 , color_2.1 , color_2.2 ) ; 
    let size_1 : usize = std::mem::size_of_val(&color_1) ; 
    let size_2 : usize = std::mem::size_of_val(&color_2) ; 
    println!("Size 1 is :{:?} and size 2 is :{:?}" , size_1 , size_2) ; 
}
____________________________________________________________________________________________________________________________
#[derive(Debug)]
struct Color1(i32 , i32 , i32) ;
#[derive(Debug)]
struct Color2(i32 , i32 , i32) ; 
fn main(){

    let color_1 : Color1 = Color1(10 , 11 , 12) ;
    let color_2 : Color2 = Color2(20 , 21 , 22) ; 
    println!("Color 1 -> :{:?}:{:?}:{:?}" ,color_1.0 , color_1.1 ,  color_1.2)  ; 
    println!("Color 2 -> :{:?}:{:?}:{:?}" ,color_2.0 , color_2.1 , color_2.2 ) ; 
    let size_1 : usize = std::mem::size_of_val(&color_1) ; 
    let size_2 : usize = std::mem::size_of_val(&color_2) ; 
    println!("Size 1 is :{:?} and size 2 is :{:?}" , size_1 , size_2) ; 
}
___________________________________________________________________________________________________________________________
#[derive(Debug , PartialEq)]
struct ZST ;
fn main(){
    let zst_1 : ZST = ZST  ;
    println!("ZST-1 -> :{:?}" , zst_1) ;
    if zst_1 == ZST {
        println!("Return true !") ; 
    }else{
        eprintln!("Return false !") ; 
    }
}
______________________________________________________________________________________________________________________
#[derive(Debug , PartialEq)]
struct ZST ;
impl ZST{
    fn print_zst(zst : &Option<Option<Vec<Option<ZST>>>>){
        match zst{
            Some(v_1)=>{
                match v_1{
                    Some(vv_1)=>{
                        match vv_1.get(0){
                            Some(f_1) =>{
                                match f_1{
                                    Some(val_1) =>{
                                        if *val_1 == ZST {
                                            println!("This is zst !") ;
                                        }else{
                                            eprintln!("This is not zst !") ; 
                                        }
                                    }
                                    None => {
                                        eprintln!("There is no value !") ; 
                                    }
                                }
                            }
                            None=>{
                                eprintln!("There is no value !") ; 
                            }
                        }
                    }
                    None =>{
                        eprintln!("There is no value !")  ; 
                    }
                }
            }
            None => eprintln!("There is no value !") ,  
        }
    }
}
fn main(){
    let zst_1 : Option<Option<Vec<Option<ZST>>>> = Some(Some(vec![Some(ZST)])) ;
    ZST::print_zst(&zst_1) ; 
}
______________________________________________________________________________________________________________________
fn main(){
    let top : (i32 , i32) = (43 , 43) ;
    let result_1 : i32 = get_area(&top) ;
    println!("Result 1 is :{:?}" , result_1) ; 
}
fn get_area(data :  &(i32  , i32))-> i32{
    return data.0 * data.1 ; 
}
______________________________________________________________________________________________________________________
#[derive(Debug)]
struct Rec{
    len : i32 ,
    wid : i32 , 
}
impl  Rec{
    fn can_hold(&self , other : &Rec) -> bool{
        if self.len > 0 && self.wid > 0 && other.len > 0 && other.wid > 0 {
            if self.len > other.len && self.wid > other.wid{
                return true ; 
            }else{
                return false ; 
            }
        }else{
            eprintln!("Please try again with a value greater then zero !") ; 
            return false ; 
        }
    }
}
fn main(){
    let rec_1 : Rec = Rec{
        len : 100 ,
        wid : 42   , 
    } ;
    let rec_2 : Rec  = Rec{
        len :43,
        wid : 10 , 
    } ;
    println!("Rec_1 can hold rec 2 is :{:?}" , rec_1.can_hold(&rec_2)) ;
}
______________________________________________________________________________________________________________________________________
#[derive(Debug)]
struct Something{
    number :i32 ,
}
impl Something{
    fn new(data : i32)-> Self{
        return Self{
            number : data , 
        } ; 
    }
}
fn main(){
    let something_1 : Something = Something::new(12 as i32) ;
    println!("Something -> :{:?}" , something_1.number) ;
}
__________________________________________________________________________________________________________________________________
