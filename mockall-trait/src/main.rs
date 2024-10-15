mod automock;
use automock::*;



fn main() {
    automock_test(); // testing automock funtionalities
    static_return_test();
    returning_non_clonable();
}
