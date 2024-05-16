use diesel::GroupedBy;
use diesel::QueryDsl;
use diesel::QueryResult;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;

use crate::models::authorized_users::AuthorizedUser;
use crate::models::roles::Role;
use crate::models::userroles::UserRole;
use crate::models::users::User;
use crate::schema::roles;
use crate::schema::users;
use crate::schema::users_roles;

pub struct AuthorizedUserRepository;

impl AuthorizedUserRepository {
    pub async fn get_all(c: &mut AsyncPgConnection) -> QueryResult<Vec<AuthorizedUser>> {
        let users = users::table.load::<User>(c).await?;

        let authorized_users = users_roles::table
            .inner_join(roles::table)
            .load::<(UserRole, Role)>(c)
            .await?
            .grouped_by(&users)
            .into_iter()
            .zip(users)
            .map(|(urrs, user)| {
                let roles = urrs.into_iter().map(|(_, role)| role).collect();
                AuthorizedUser { user, roles }
            })
            .collect();

        Ok(authorized_users)
    }
}
