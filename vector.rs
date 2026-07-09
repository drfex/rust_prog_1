fn main(){
    let array : [i32 ; 4] = [10 , 20 , 30 , 40] ;
    let vector : Vec<i32> = Vec::from(&array) ;
    println!("Vector is :{:?}" , vector) ;  
}
________________________________________________________________________________________
fn main(){
    let vector : [i32  ; 4] = [10 , 20 , 30 , 40] ;

    let ref x : &[i32 ; 4] = &vector ;
    println!("X is :{:?}" , x) ; 
}

________________________________________________________________________________
fn main(){
    let vector : Vec<i32> = vec![120 , 30 , 423 , 342] ;
    let sec : Option<&i32> = vector.get(1) ;
    match sec{
        Some(v_1 ) => {
            println!("The sec 1 is -> :{:?}" , v_1) ; 
        }
        None =>{
            eprintln!("There is no vakl !")  ; 
        }
    }
}
_____________________________________________________________________

