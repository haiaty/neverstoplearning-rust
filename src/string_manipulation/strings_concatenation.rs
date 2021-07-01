


fn main() {


    //====================
    // using format
    //========================

    let mut _a = "a".to_string();
    let _b = "b".to_string();
    let _c = format!("{}{}", _a, _b);

    println!("{}", _c);

    //====================
    // using + operator:
    //========================

    let mut _a = "a".to_string();
    let _c = "c".to_string();

    println!("{}", _a + &_c);

    //====================
    // using push_str():
    //========================

    let mut _a = "a".to_string();
    let _b = "b".to_string();

    _a.push_str(&_b);

    println!("{}", _a);


    //=================
    // using concat!() - ATTENTION: only with literals
    //=====================

     concat!("a","other", true);



}