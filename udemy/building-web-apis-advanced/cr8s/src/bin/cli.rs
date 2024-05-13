use clap::Arg;
use clap::Command;

extern crate cr8s;

#[tokio::main]
async fn main() {
    let username = Arg::new("username").short('u').required(true);
    let password = Arg::new("password").short('p').required(true);
    let roles = Arg::new("roles")
        .short('r')
        .required(true)
        .num_args(1..)
        .value_delimiter(',');
    let create = Command::new("create")
        .about("Create a new user")
        .arg_required_else_help(true)
        .arg(username)
        .arg(password)
        .arg(roles);

    let list = Command::new("list").about("List all users");

    let id = Arg::new("id").required(true);
    let delete = Command::new("delete")
        .about("Delete user by id")
        .arg_required_else_help(true)
        .arg(id);

    let users = Command::new("users")
        .about("User management")
        .arg_required_else_help(true)
        .subcommand(create)
        .subcommand(list)
        .subcommand(delete);

    let matches = Command::new("cr8s")
        .about("A CLI for the Cr8s API")
        .arg_required_else_help(true)
        .subcommand(users)
        .get_matches();

    match matches.subcommand() {
        Some(("users", submatches)) => match submatches.subcommand() {
            Some(("create", create_matches)) => {
                let username = create_matches
                    .get_one::<String>("username")
                    .unwrap()
                    .to_owned();
                let password = create_matches
                    .get_one::<String>("password")
                    .unwrap()
                    .to_owned();
                let role_codes = create_matches
                    .get_many::<String>("roles")
                    .unwrap()
                    .map(|s| s.to_owned())
                    .collect();

                cr8s::commands::create_user(username, password, role_codes).await
            }

            Some(("list", _)) => cr8s::commands::list_users().await,

            Some(("delete", delete_matches)) => {
                let id = delete_matches.get_one::<i32>("id").unwrap().to_owned();

                cr8s::commands::delete_user(id).await
            }

            _ => unreachable!(),
        },

        _ => unreachable!(),
    }
}
