// pub fn get_life_time_var() {
//     let r: &i32;
//     {
//         let x = 5; // x dead if outside scope
//         r = &x; // r not ref to x when outside scope
//     }

//     println!("{}", r);
// }

pub fn get_max_numbers<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x
    } else {
        y
    }
}
