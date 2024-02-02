pub fn get_scope() {
    let x: i32 = 17;
    {
        let y: i32 = 3;
        println!("The value of x in scope is {} and value of y is {}", x, y);
    }
    println!("The value of x out scope is {}", x);
}
