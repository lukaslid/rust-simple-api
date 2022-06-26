use crate::errors::user::UserError;
use crate::schema::user;
use argon2::Config;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "user"]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct NewUser {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3, max = 20))]
    pub name: String,
    #[validate(length(min = 3, max = 20))]
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

impl User {
    pub fn get_all(conn: &PgConnection) -> Result<Vec<Self>, UserError> {
        match user::table.load::<User>(conn) {
            Ok(users) => Ok(users),
            Err(_) => Err(UserError::InternalError),
        }
    }

    pub fn get(conn: &PgConnection, id: Uuid) -> Result<Self, UserError> {
        match user::table.filter(user::id.eq(id)).first(conn) {
            Ok(user) => Ok(user),
            Err(diesel::result::Error::NotFound) => Err(UserError::UserNotFoundError),
            Err(_) => Err(UserError::InternalError),
        }
    }

    pub fn create(conn: &PgConnection, user_data: NewUser) -> Result<Self, UserError> {
        let mut user = User::from(user_data);
        user.hash_password()?;

        match diesel::insert_into(user::table)
            .values(&user)
            .get_result(conn)
        {
            Ok(user) => Ok(user),
            Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            )) => Err(UserError::EmailAlreadyExistsError { email: user.email }),
            Err(_) => Err(UserError::InternalError),
        }
    }

    pub fn login(conn: &PgConnection, login_data: LoginData) -> Result<Self, UserError> {
        if let Ok(user_to_verify) = user::table
            .filter(user::email.eq(&login_data.email))
            .get_result::<User>(conn)
        {
            if user_to_verify.password.is_empty() {
                return Err(UserError::InvalidCredentials);
            }

            if user_to_verify.verify_password(&login_data.password.as_bytes())? {
                return Ok(user_to_verify);
            }
        }

        Err(UserError::InvalidCredentials)
    }

    pub fn hash_password(&mut self) -> Result<(), UserError> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let cfg = Config::default();

        if let Ok(hashed_password) = argon2::hash_encoded(self.password.as_bytes(), &salt, &cfg) {
            self.password = hashed_password;
            Ok(())
        } else {
            Err(UserError::InternalError)
        }
    }

    pub fn verify_password(&self, password: &[u8]) -> Result<bool, UserError> {
        argon2::verify_encoded(&self.password, password).map_err(|_e| {
            println!("{}", _e);
            UserError::InvalidCredentials
        })
    }
}

impl From<NewUser> for User {
    fn from(user: NewUser) -> Self {
        User {
            id: Uuid::new_v4(),
            email: user.email,
            name: user.name,
            password: user.password,
            created_at: Utc::now().naive_utc(),
        }
    }
}
