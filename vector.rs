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
fn main(){
    let vec : Vec<Vec<Vec<Vec<String>>>> = vec![vec![vec![vec![String::from("Kohee") , String::from("Kiaff") , String::from("Vogi")]]]] ;
    match vec.get(0){
        Some(c_1) =>{
            match c_1.get(0){
                Some(c_2) => {
                    match c_2.get(0){
                        Some(c_3) =>  {
                            for i in c_3.iter(){
                                println!("Vec :{:?}" , i) ;
                            }
                        }
                        None => {} ,
                    }
                }
                None => {} ,
            }
        }
        None =>{} ,
    }
}

____________________________________________________________________________________________________________________________________________________________
fn main(){
    let arr_1 : [i32 ; 4] = [10 , 20 , 30 , 30] ;
    let arr_2 : [i32 ; 4] = [10 , 30 , 40  , 40] ;
    let vec : Vec<&[i32 ; 4]> = vec![&arr_1 , &arr_2] ;
    println!("The vec is :{:?}"  , vec) ;
}
______________________________________________________________________________________________________________________________________________
fn main(){
    let arr_1 :[Vec<Option<i32>> ; 4] = [vec![Some(10)] , vec![Some(2)] , vec![Some(43)] , vec![Some(32)]] ;
    let arr_2 :[Vec<Option<i32>> ; 4] = [vec![Some(3)] , vec![Some(42)], vec![Some(23)] , vec![Some(42)]] ;
    let vec_1 :Vec<&[Vec<Option<i32>> ; 4]> = vec![&arr_1 , &arr_2] ;
    println!("The vector is :{:?}" , vec_1) ;       
}
______________________________________________________________________________________________________________________________________________________
fn main(){
   let  vec_2d : Vec<[i32 ; 2]> = [[10 , 20] , [32 , 42]].to_vec() ;
   for i in vec_2d{
       println!("2d vec is :{:?}" , i) ; 
   } 
}
___________________________________________________________________________________________________________________________________________________
fn main(){
    let vec:  Vec<[i32 ; 2]> = vec![[10 , 20] , [40 , 32]] ;
    for i in vec.iter(){
     match  i.get(1){
            Some(vv_1) =>{
                println!("Vec 1 is -> :{:?}" , vv_1) ; 
            }
            None=> {} , 
        }
    }
}
________________________________________________________________________________________________________________________________________________
fn main(){
    let vec : Vec<Vec<i32>> = vec![
        vec![120 , 30 , 40] ,
        vec![12 , 20  , 30] , 
    ] ;
    for (row_idx , row) in vec.iter().enumerate(){
        for(col_idx , col) in row.iter().enumerate(){
            println!("row index :{:?} col index :{:?} val :{:?}" , row_idx , col_idx  , col) ;
        }
    } 
}
_____________________________________________________________________________________________________________________
fn main(){
    let vector : Vec<Vec<&i32>> = vec![
        vec![&120 , &0 , &4 , &42] ,
        vec![&32 , &43  , &3] ,
        vec![&32 , &43 , &4] ,
    ] ;
    for (row_idx , row) in vector.iter().enumerate(){
        for (col_idx , col) in row.iter().enumerate(){
            println!("Row :{:?} Col  :{:?} and value :{:?}" , row_idx , col_idx , col) ;
        }
    }
}
_____________________________________________________________________________________________________________________________________
fn main(){
    let arr : [[i32 ;4] ; 3] = [[10 , 20 , 3 , 2] , [2, 0 , 4 , 2] , [3 , 2 , 4, 3]] ;
    for i in arr.iter(){
        for j in i.iter(){
            println!("j is :{:?}" , j) ;
        }
    }
}
_______________________________________________________________________________________________________________________________________________
fn main(){
    let arr : [[i32 ;4] ; 3] = [[10 , 20 , 3 , 2] , [2, 0 , 4 , 2] , [3 , 2 , 4, 3]] ;
    for i in arr.iter(){
        print!("\n") ;
        for j in i.iter(){
            println!("->  :{:?}" , j) ;
        }
    }
}
________________________________________________________________________________________________________________________________________________
fn main(){
    let vec : Vec<[Vec<Vec<i32>> ; 2]>  = vec![
        [vec![vec![10]] , vec![vec![22]]] ,
        [vec![vec![32]] , vec![vec![32]]] , 
    ];
    println!("vector is :{:?}" , vec) ; 
}
_______________________________________________________________________________________________________________________________
fn main(){
    let vec: Vec<Vec<[Vec<i32> ; 3]>> = vec![vec![
        [vec![10] , vec![120] , vec![32]] ,
        [vec![23] , vec![34] ,  vec![23]] ,
    ]] ;
    for i_1 in vec.iter() {
        for i_2 in i_1.iter(){
            for i_3 in i_2.iter(){
               for i_4 in i_3.iter() {
               println!("Val :{:?} " , i_4) ;
               }
           }
        }
    }
}
________________________________________________________________________________________________________________________________________________
fn main(){
    let vec : Vec<Vec<[Vec<i32> ; 2]>> = vec![vec![
        [vec![120] , vec![120]] ,
        [vec![22] , vec![42]] , 
    ]] ;
    for i in 0..vec.len(){
        match vec.get(i) {
            Some(first_layer) =>{
                for col_index in 0..first_layer.len(){
                    match first_layer.get(col_index){
                        Some(inner_1) =>{
                            for inner_val in 0..inner_1.len(){
                                match inner_1.get(inner_val){
                                    Some(inner_2) => {
                                        for inn in 0..inner_2.len(){
                                            match inner_2.get(inn){
                                                Some(main_val) => {
                                                    println!("Main val -> :{:?}" , main_val) ;
                                                }
                                                None => {} , 
                                            }
                                        }
                                    }
                                    None => {} , 
                                }
                            }
                        }
                        None =>{} , 
                    }
                }
            }
            None => {} , 
        }
    }
}
_____________________________________________________________________________________________________________________________________________________
fn main(){
    let vec : Vec<Vec<Vec<[Vec<i32> ; 3]>>> = vec![vec![vec![
        [vec![21] , vec![32] , vec![21]] ,
        [vec![312] , vec![23] , vec![23]] , 
    ]]] ;
    for    i   in     0..vec.len(){
        match vec.get(i) {
            Some(warp_2) =>{
                for warp_index_1 in 0..warp_2.len(){
                    match warp_2.get(warp_index_1){
                        Some(warp_3) =>{
                            for inner_1 in 0..warp_3.len(){
                                match warp_3.get(inner_1){
                                    Some(inner_2) =>{
                                        for inner_3 in 0..inner_2.len(){
                                            match inner_2.get(inner_3){
                                                Some(inner_4) =>{
                                                    for inner_5 in 0..inner_4.len(){
                                                        match inner_4.get(inner_5){
                                                            Some(final_1) =>{
                                                                println!("F-> :{:?}" , final_1) ; 
                                                            }
                                                            None => {} , 
                                                        }
                                                    }
                                                }
                                                None => {} , 
                                            }
                                        }
                                    }
                                    None => {} , 
                                }
                            }
                        }
                        None => {} ,
                    }
                }
            }
            None => {} , 
        }
    }
}
__________________________________________________________________________________________________________________________________________

