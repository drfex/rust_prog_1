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
very bad practice , this only covers one side 
#[derive(Debug)]
struct Info1{
    name :Option<String> ,
    age : Option<i32> , 
}
#[derive(Debug)]
struct Info2{
    name : Option<String> ,
    age : Option<i32> , 
}
#[derive(Debug)]
enum AddInfo{
    Type1(Info1) ,
    Type2(Info2) , 
}
impl AddInfo{
    fn print_info(info : &Vec<Option<Vec<Option<AddInfo>>>>){
        match info.get(0){
            Some(f_1)=>{
                match f_1{
                    Some(first_1)=> {
                        match first_1.get(0){
                            Some(sec_1) =>{
                                match sec_1{
                                    Some(f_1)=>{
                                        match f_1{
                                        AddInfo::Type1(val_1)=> {
                                            match &val_1.name {
                                                Some(nn_1) =>{
                                                    println!("Name is :{:?}" , nn_1) ; 
                                                }
                                                None => {
                                                    eprintln!("There is no value !") ;
                                                }
                                            }
                                            match &val_1.age{
                                                Some(gg_1) =>{
                                                    println!("Age is :{:?}" , gg_1) ; 
                                                }
                                                None => {
                                                    eprintln!("There is no value !") ; 
                                                }
                                            }
                                        }
                                        AddInfo::Type2(val_2) =>{
                                            match &val_2.name {
                                                Some(nn_2)=>  {
                                                    println!("name is :{:?}" , nn_2) ; 
                                                }
                                                None => {
                                                    eprintln!("There is no value !") ; 
                                                }
                                            }
                                            match  &val_2.age{
                                                Some(gg_2) => {
                                                    println!("Age is :{:?}" , gg_2) ; 
                                                }
                                                None => {
                                                    eprintln!("There is no age value !") ; 
                                                }
                                            }
                                         }
                                      }
                                            
                                   }
                                   None => {
                                       eprintln!("There is no value !") ; 
                                   }
                                }
                            }
                            None => {
                                eprintln!("There is no value !") ; 
                            }            
                        }
                    }
                    None => {
                        eprintln!("There is no value !") ; 
                    }
                }
            }
            None => {
                eprintln!("There is no value!") ; 
            }
        }
    }
}
fn main(){
    let info_1 : Option<Vec<Option<AddInfo>>> = Some(vec![
        Some(
            AddInfo::Type1(Info1{
                name : Some(String::from("Kiaff !")) ,
                age : Some(3 as i32) , 
            })
        ) ,
        Some(
            AddInfo::Type2(Info2{
                name: Some(String::from("Juggy !")) ,
                age : Some(43 as i32) , 
            })
        )
    ]) ;
    AddInfo::print_info(&vec![info_1]) ; 
}
___________________________________________________________________________________________________________________________________________
fn main(){
    let vec : Vec<Vec<Option<String>>> = vec![vec![Some(String::from("Kiaff")) ,Some(String::from("Kiaff"))]] ;
    match vec.get(0){
        Some(f_1) => {
            match f_1.get(0){
                Some(f_2) =>{
                    match f_2{
                        Some(v_1) =>{
                            println!("name is :{:?}" , v_1) ;
                        }
                        None => {} ,
                    }
                }
                None => {} ,
            }
        }
        None => {} ,
    }
}
______________________________________________________________________________________________________________________________________________
fn main(){
    let vec : Option<Option<Vec<Option<i32>>>> = Some(Some(vec![Some(10) , Some(20) , Some(20)])) ;
    match vec{
        Some(v_1) => {
            match  v_1{
                Some(v_2) => {
                    match v_2.get(0){
                        Some(vv_1) => {
                            match vv_1{
                                Some(name) =>{
                                    println!("Number is :{:?}" , name) ;
                                }
                                None => {
                                    eprintln!("There is no value !") ;
                                }
                            }
                        }
                        None => {
                            eprintln!("There is no value !") ;
                        }
                    }
                }
                None => {
                    eprintln!("There is no value !") ;
                }
            }
        }
        None => {
            eprintln!("There is no value !") ;
        }
    }
}
_______________________________________________________________________________________________________________________________________________
fn main(){
    let vec : Vec<Vec<Vec<i32>>> = vec![vec![vec![10 , 20 , 30 , 40 , 50 , 60]]] ;
    match vec.get(0){
        Some(vec_1) =>{
            match vec_1.get(0){
                Some(vec_2) =>{
                    match vec_2.get(0){
                        Some(final_1) =>{
                            println!("Final value :{:?}" ,  final_1) ;
                        }
                        None => {
                            eprintln!("There is no value !") ;
                        }
                    }
                }
                None =>{} ,
            }
        }
        None => {} ,
    }
}
___________________________________________________________________________________________________________________________________________________
fn main(){
    let vec :Vec<Vec<Vec<i32>>> = vec![vec![vec![10 , 20 , 30 , 40 , 50 ,60]]] ;
    match vec.get(0){
        Some(v_1) => {
            match v_1.get(0){
                Some(vv_1) =>{
                    for i in vv_1.iter(){
                        println!("Vec :{:?}" , i) ; 
                    }
                }
                None=> {} ,
            }
        }
        None => {} , 
    }
}
_____________________________________________________________________________________________________________________________________________

