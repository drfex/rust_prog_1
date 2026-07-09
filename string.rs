________________________________________________________
fn main(){
    let name_1 : String = String::from("kohee") ;
    let name_2 : String = "kohee".to_string() ;
    let name_3 : &str = "kohee" ;
    let name_4 : &'static str = "kohe"  ;  
    println!("Name is :{:?}" , name_1) ;
    println!("Name 2 is :{:?}" ,name_2) ; 
    println!("Name 3 is :{:?}" , name_3) ; 
    println!("Name 4 is :{:?}" , name_4) ;
}
________________________________________________________________________________________________________________________________
fn main(){

    let name = String::from("KOhee") ;
    let ko : String = name[0..2].to_string() ;
    let  full : String  = name[..].to_string() ;
    let full_2 : String = name[0..name.len()].to_string() ;
    println!("Name is :{:?} and ko is :{:?} and full string is :{:?}"  ,name  , ko , full) ;
    println!("Full 2 is :{:?}" , full_2) ;
}
_______________________________________________________________________________________________________________________________
fn main(){
    let word : String = String::from("SSKTJL") ;
    for i in word.chars() {
        println!("I is :{:?}" , i) ; 
    }
}
_______________________________________________________________________________________________________________________________
fn main(){
    let word : String =  String::from("Kohee and") ;
    let start_with : bool = word.starts_with("Kohee") ;
    let end_with  : bool = word.ends_with("and") ;
    let contains : bool = word.contains("and") ;
    let ano : bool  = word.contains(" ") ;
    println!("Word starts with kohee is :{:?}" , start_with) ;
    println!("Word ends with kohee is :{:?}" , end_with) ;
    println!("Word contains and is :{:?}" , contains) ;
    println!("Word contains space is also :{:?}" , ano) ;  
}
_____________________________________________________________________________________________________________________________
fn main(){
    println!("Input-> ") ;
    let mut string : String = String::new() ;
    std::io::stdin().read_line(&mut string).expect("Failed to do it !") ;
    let input = string.trim() ;
    println!("The input is :{:?}" , input) ;
}
_________________________________________________________________________________________________________________________________
