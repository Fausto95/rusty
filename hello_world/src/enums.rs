pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub fn go_to(direction: Direction) {
    match direction {
        Direction::Up => println!("Going up!"),
        Direction::Down => println!("Going down!"),
        Direction::Left => println!("Going left!"),
        Direction::Right => println!("Going right!")
    }
}
