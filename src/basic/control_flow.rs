use std::num::ParseIntError;

pub fn fn_if() {
    let x = 20;
    if x == 20 {
        println!("if x: {}", x);
    }
}

pub fn match_result() -> Result<(), ParseIntError> {
    let number_str = "20";
    let number = match number_str.parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };
    println!("result: {}", number);
    Ok(())
}

pub fn match_option() {
    let x = Some(2);
    let y = None;

    let xx = match x {
        Some(v) => v,
        None => 0,
    };

    let yy = match y {
        Some(v) => v,
        None => "None",
    };

    println!("Some x: {} None :{}", xx, yy);
}

pub fn for_loop_num() {
    for x in 1..10 {
        println!("{}", x);
    }
}
pub fn for_loop_num_index() {
    for (x, i) in (1..10).enumerate() {
        println!("i={} v={}", i, x);
    }
}

pub fn for_loop_vec() {
    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for (index, val) in list.iter().enumerate() {
        println!("index: {}, value:{}", index, val);
    }
}

pub fn for_loop_iter() {
    let list = vec!["rust", "actix-web", "axum"];
    for item in list {
        println!("item: {}", item);
    }
}
