use super::{roles::Role, users::User};

#[derive(Debug)]
pub struct AuthorizedUser {
    pub user: User,
    pub roles: Vec<Role>,
}
