mod functions;
mod arithmetic;
mod control_flow;
mod pattern_matching;
mod loops;
mod enums;
mod structures;
mod tuples;
mod expressions;
mod ownership;
mod implementation;
mod vectors;
mod strings;

use functions::display_name;
use control_flow::control_flow;
use pattern_matching::pattern_matching;

use loops::{repetition, while_loop};

fn main() {
    display_name();

    // Module arithmetic.arithmetic()
    arithmetic::arithmetic();

    control_flow();
    pattern_matching();
    repetition();
    while_loop();

    enums::go_to(enums::Direction::Up);
    enums::go_to(enums::Direction::Down);
    enums::go_to(enums::Direction::Left);
    enums::go_to(enums::Direction::Right);

    structures::create_box();
    structures::create_person();
    structures::create_drink();

    tuples::start();

    expressions::expressions();
    expressions::get_access_level();

    let day_week = ownership::WeekDay::Monday;
    ownership::display_week_day(&day_week);
    ownership::display_week_day(&day_week);

    implementation::log_person();

    vectors::get_boxes();

    strings::print_frameworks();
}
