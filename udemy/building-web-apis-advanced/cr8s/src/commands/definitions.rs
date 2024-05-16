use clap::value_parser;
use clap::Arg;
use clap::Command;

pub fn cr8s() -> Command {
    Command::new("cr8s")
        .about("A CLI for the Cr8s API")
        .arg_required_else_help(true)
        .subcommand(roles())
        .subcommand(users())
}

fn roles() -> Command {
    let create = Command::new("create")
        .about("Create a new role")
        .arg_required_else_help(true)
        .arg(Arg::new("code").short('c').required(true))
        .arg(Arg::new("name").short('n').required(true));

    let list = Command::new("list").about("List all roles");

    Command::new("roles")
        .about("Role management")
        .arg_required_else_help(true)
        .subcommand(create)
        .subcommand(list)
}

fn users() -> Command {
    let create = Command::new("create")
        .about("Create a new user")
        .arg_required_else_help(true)
        .arg(Arg::new("username").short('u').required(true))
        .arg(Arg::new("password").short('p').required(true))
        .arg(
            Arg::new("roles")
                .short('r')
                .required(true)
                .num_args(1..)
                .value_delimiter(','),
        );

    let delete = Command::new("delete")
        .about("Delete user by id")
        .arg_required_else_help(true)
        .arg(
            Arg::new("id")
                .required(true)
                .value_parser(value_parser!(i32)),
        );

    let list = Command::new("list").about("List all users");

    Command::new("users")
        .about("User management")
        .arg_required_else_help(true)
        .subcommand(create)
        .subcommand(delete)
        .subcommand(list)
}
