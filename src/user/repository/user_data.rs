use crate::DBPool;
use crate::shared::error_handler::ApiError;
use crate::user::dto::RegisterUser;
use crate::user::repository::model::User;

pub struct UserData {
    db: DBPool,
}

impl UserData {
    pub fn new(db: DBPool) -> Self {
        Self { db }
    }
}

impl UserData {
    pub async fn create_user(&self, data: &RegisterUser, password: String) ->
    Result<User, ApiError> {
        let query = r#"insert into users(name, surname, patronymic, email, password)
         VALUES ($1, $2, $3, $4, $5) returning *"#;

        let result = sqlx::query_as::<_, User>(query)
            .bind(data.name.clone().unwrap())
            .bind(data.surname.clone().unwrap())
            .bind(data.patronymic.clone().unwrap())
            .bind(data.email.clone().unwrap())
            .bind(password).fetch_one(&self.db).await?;

        sqlx::query("insert into carts(user_id) values($1)").bind(&result.id)
            .execute(&self.db).await?;

        Ok(result)
    }

    pub async fn update_token(&self, id: &i32, token: &String) -> Result<User, ApiError> {
        let query = r#"update users set token = $1 where id = $2 returning *"#;

        let result = sqlx::query_as::<_, User>(query)
            .bind(token)
            .bind(id).fetch_one(&self.db).await?;

        Ok(result)
    }

    pub async fn user_by_email(&self, email: String) -> Result<User, ApiError> {
        let query = r#"select * from users where email = $1"#;

        let result = sqlx::query_as::<_, User>(query)
            .bind(email).fetch_one(&self.db).await?;

        Ok(result)
    }
}