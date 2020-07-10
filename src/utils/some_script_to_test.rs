

fn main () {


    // perchè String, array of chars, si trova nell'heap
    // per valori nello stack non serve
    let mut ve  = vec![1, 2, 3];


    ve.push(5);

    a_func(&mut ve);


    println!("from func {:#?}", ve);


    
}


fn a_func(ve: &mut Vec<u32>) {


    ve.push(6);
    println!("from func {:#?}", ve);

}