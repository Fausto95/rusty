// enum Nationality {
//     Angolan,
// }

pub fn make_tuple() -> (i32, i8, u32) {
    (10_000, 27, 2_000)
}

pub fn start() {
    let result = make_tuple();
    let (a, b, c) = make_tuple();
    let (x, y) = (10, 20);

    println!("{:?}, {:?}", x, y);

    println!("Tuples - {:?}, {:?}", a, result.0);
    println!("Tuples - {:?}, {:?}", b, result.1);
    println!("Tuples - {:?}, {:?}", c, result.2);

    //let (name, nationality) = ("Faustino", Nationality::Angolan);
}