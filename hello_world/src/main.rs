mod functions;
mod arithmetic;
mod control_flow;
mod pattern_matching;

use functions::display_name;
use control_flow::control_flow;
use pattern_matching::pattern_matching;



fn main() {
    display_name();

    // Module arithmetic.arithmetic()
    arithmetic::arithmetic();

    control_flow();
    pattern_matching();
}
