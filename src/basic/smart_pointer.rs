// smart pointer
// Box<T>, Rc<T>, RefCell<T>

use std::rc::Rc;

#[derive(Debug)]
pub enum List {
    // Box<List> --> *List
    Node(i32, Box<List>),
    Nil,
}

pub fn get_list_box() {
    let lists = List::Node(1, Box::new(List::Node(2, Box::new(List::Nil))));
    let list_box = Box::new(2);
    println!("Box list pointer ==> {:?}", lists);
    println!("Box pointer ==> {}", list_box);
}

pub fn get_rc_pointer() {
    let x = Rc::new(Box::new(10));
    let y = Rc::clone(&x);
    let z = Rc::clone(&x);

    println!("Rc Pointer ==> x: {}, y: {}, z: {}", x, y, z);
}
