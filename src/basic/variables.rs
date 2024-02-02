use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub fn var_binding() {
    let x = 7;

    println!("var binding: {}", x);

    // desctruct
    let (a, b) = (10, 20);
    println!("var decstruct: {},{}", a, b);

    let msg: String = "sutin".parse().unwrap();
    let y = 20.6;
    println!(
        "{:?} is type {} len: {} cap: {}",
        &msg,
        type_of(&msg),
        msg.len(),
        msg.capacity()
    );
    println!("{:?} is type {}", &y, type_of(&y));
}

pub fn mutable_var() {
    let mut x = 20;
    x = x + 20;
    println!("mutable var:  x + 20 = {}", x);
}
