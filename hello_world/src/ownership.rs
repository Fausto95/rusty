pub enum WeekDay {
    Monday,
    Friday,
}

pub fn display_week_day(day: &WeekDay) {
    match day {
        WeekDay::Monday => println!("Today is Monday"),
        WeekDay::Friday => println!("Today is Friday"),
    }
}
