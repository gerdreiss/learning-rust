extern crate cr8s;

use cr8s::commands::definitions;
use cr8s::commands::handlers;

#[tokio::main]
async fn main() {
    match definitions::cr8s().get_matches().subcommand() {
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

                handlers::create_user(username, password, role_codes).await
            }

            Some(("list", _)) => handlers::list_users().await,

            Some(("delete", delete_matches)) => {
                let id = delete_matches.get_one::<i32>("id").unwrap().to_owned();

                handlers::delete_user(id).await
            }

            _ => unreachable!(),
        },

        Some(("roles", submatches)) => match submatches.subcommand() {
            Some(("create", create_matches)) => {
                let code = create_matches.get_one::<String>("code").unwrap().to_owned();
                let name = create_matches.get_one::<String>("name").unwrap().to_owned();

                handlers::create_role(code, name).await
            }

            Some(("list", _)) => handlers::list_roles().await,

            _ => unreachable!(),
        },

        _ => unreachable!(),
    }
}
