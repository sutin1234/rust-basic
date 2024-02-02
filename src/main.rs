use std::{sync::mpsc, thread, time::Duration};

use rust_basic::basic::{
    control_flow::{
        fn_if, for_loop_iter, for_loop_num, for_loop_num_index, for_loop_vec, match_option,
        match_result,
    },
    enums::PlayerEnum,
    fucntions::{add_one, add_two},
    generics::Point,
    hello::say_hello,
    life_times::get_max_numbers,
    shadow_scope::get_scope,
    smart_pointer::{get_list_box, get_rc_pointer},
    structs::Monster,
    traits::{excute_attach, Mage, Warrior},
    variables::{mutable_var, var_binding},
    vectors::{get_collect, get_find_vect, get_list_iter, get_vec, get_vect_map},
};

fn main() {
    // [1] say_h
    say_hello();

    // [2] if
    fn_if();

    // [3] match Result
    match_result().unwrap();

    // [4] match some\
    match_option();

    // [5] loop n
    for_loop_num();

    // [6] loop n with index
    for_loop_num_index();

    // [7] loop n with vector
    for_loop_vec();

    // [8] loop iter
    for_loop_iter();

    // [9] variables
    var_binding();

    // [10] mut variables
    mutable_var();

    // [11] scope & shadow
    get_scope();

    // [12] function
    let added_one = add_one(20);
    println!("add one no return syntax: {}", added_one);

    let added_two = add_two(27);
    println!("add two return syntax: {}", added_two);

    // [13] vector & iter
    let list = get_vec();
    println!("list vector: {:?}", list);

    // loop with iter()
    get_list_iter();

    // list form collect
    get_collect();

    // list from map
    get_vect_map();

    // get_find
    get_find_vect();

    // [14] struct
    let monster = Monster {
        name: "crap".to_owned(),
        level: 1,
        attach: 100,
    };
    println!("monster: {:?}\n", monster);

    let mut monster = Monster::new("crap".to_owned(), 4, 250);
    monster.attach();
    monster.reduce_attach(20);
    println!("reduce_attach: {:?}\n", monster.attach);

    monster.level_up(7);
    println!("level up: {:?}\n", monster.level);

    // [15] trait
    let mage = Mage {
        name: "my Mage".to_owned(),
        level: 10,
    };
    let warrior = Warrior {
        name: "my Warrior".to_owned(),
        level: 10,
    };

    excute_attach(&mage);
    excute_attach(&warrior);

    // [16] enum;
    let player_enum = PlayerEnum::MAGE(20);
    println!(
        "enum {:?}, {:?}",
        PlayerEnum::MAGE(10),
        PlayerEnum::WARRIOR(15)
    );
    player_enum.get_weapon();

    // [17] generics
    let p1 = Point { x: 23.7, y: 21.9 };
    let p2 = Point { x: 33, y: 29 };
    println!("A ({},{})", p1.get_x(), p1.get_y());
    println!("B ({},{})", p2.get_x(), p2.get_y());

    // [18] lifetime
    let x = 10;
    let y = 20;
    let max_nums = get_max_numbers(&x, &y);
    println!("life time {}", max_nums);

    //[19] smart pointer
    get_list_box();
    get_rc_pointer();

    // [20] thread & concurrency
    thread::spawn(|| {
        for i in 0..3 {
            println!("thread #1: {}", i);
        }
    });

    thread::spawn(|| {
        for i in 0..2 {
            println!("thread #2 : {}", i);
        }
    });
    println!("thread finished");

    // [21] channel thread
    // TX = sender, RX = receriver
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let messages = vec![
            "hello".to_owned(),
            "from".to_owned(),
            "the".to_owned(),
            "thread".to_owned(),
        ];
        for msg in messages {
            println!("sended: {}", msg);
            tx.send(msg).unwrap();
        }
        thread::sleep(Duration::from_secs(2));
    });

    thread::spawn(move || {
        let messages = vec![
            "hello 2".to_owned(),
            "from 2".to_owned(),
            "the 2".to_owned(),
            "thread 2".to_owned(),
        ];
        for msg in messages {
            println!("sended: {}", msg);
            tx1.send(msg).unwrap();
        }
        thread::sleep(Duration::from_secs(1));
    });

    for received in rx {
        println!("rx Got: {}", received);
    }
}
