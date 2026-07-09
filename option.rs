fn main(){
    let some_number_1 : std::option::Option<i32> = Some(20) ;
    let none_number_1 : std::option::Option<i32> = None  ;
    match some_number_1{
        Some(val_1) => {
            println!("val 1 -> :{:?}" , val_1) ; 
        }
        None => {
            eprintln!("There is no value !") ; 
        }
    }
    match none_number_1{
        Some(val_1) =>{
            println!("val 1 -> :{:?}" , val_1) ; 
        }
        None => {
            println!("There is no value !") ; 
        }
    }
}
____________________________________________________________________________________________________________________________


