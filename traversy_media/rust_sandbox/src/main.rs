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
mod structs;
mod enums;
mod cli;

fn main() {
    print_l("print");
    print::run();
    print_l("vars");
    vars::run();
    print_l("types");
    types::run();
    print_l("strings");
    strings::run();
    print_l("tuples");
    tuples::run();
    print_l("arrays");
    arrays::run();
    print_l("vectors");
    vectors::run();
    print_l("conditionals");
    conditionals::run();
    print_l("loops");
    print_l("loops");
    loops::run();
    print_l("functions");
    functions::run();
    print_l("pointer_refs");
    pointer_refs::run();
    print_l("structs");
    structs::run();
    print_l("enums");
    enums::run();
    print_l("cli");
    cli::run();
}
fn print_l(name: &str) {
    println!("\n");
    for x1 in 0..40 {
        print!("-");
        if x1 == 5 {
            print!("| {: <15}|", name);
        }
    }
    println!("\n");
}