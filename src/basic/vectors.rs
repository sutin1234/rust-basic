pub fn get_vec() -> Vec<&'static str> {
    let mut list = Vec::new();
    for item in vec!["rust", "actix-web", "wrap", "axum"] {
        list.push(item);
    }
    println!("apps: {:?}", list);

    list.pop();
    println!("after pop apps: {:?}", list);

    list.remove(0);
    println!("after shift apps: {:?}", list);

    return list;
}

pub fn get_list_iter() {
    let lists = vec!["list 1", "list 2", "list 3"];
    for list in lists.iter() {
        println!("vec!: {}", list);
    }
}

pub fn get_collect() {
    let nums = (1..6).collect::<Vec<i32>>();
    for num in nums {
        println!("collect: {}", num);
    }
}

pub fn get_vect_map() {
    let nums = (1..6).map(|x| x + 1);
    for num in nums {
        println!("map: {}", num);
    }
}

pub fn get_find_vect() {
    let nums = (1..6).find(|x| *x >= 4);

    match nums {
        Some(x) => println!("found match {}", x),
        None => println!("not found"),
    };
}
