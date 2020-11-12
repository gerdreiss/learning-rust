mod arrays;
mod print;
mod strings;
mod tuples;
mod types;
mod vars;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointer_refs;

fn main() {
    print::run();
    vars::run();
    types::run();
    strings::run();
    tuples::run();
    arrays::run();
    vectors::run();
    conditionals::run();
    loops::run();
    functions::run();
    pointer_refs::run();
}
