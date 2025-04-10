use chrono::{DateTime, Utc};
use sqlx::Row;

#[derive(Debug, Clone, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub age: u32,

    #[serde(with = "chrono::serde::ts_seconds")]
    pub ctime: DateTime<Utc>,
}

impl User {
    pub async fn get_user(db: &crate::db::DB, id: u32) -> Result<User, warp::Rejection> {
        // let user: User = sqlx::query_as("SELECT * FROM user WHERE id=?")
        //     .bind(id)
        //     .fetch_one(db)
        //     .await
        //     .unwrap();

        let row = sqlx::query("SELECT * FROM user WHERE id=?")
            .bind(id)
            .fetch_one(db)
            .await
            .unwrap();
        println!("{:?}", row);

        let ctime: DateTime<Utc> = row.get("ctime");
        println!("{:?}", ctime);

        let user = User {
            id: row.get("id"),
            name: row.get("name"),
            age: row.get("age"),
            ctime: row.get("ctime"),
        };

        Ok(user)
    }

    pub async fn create_user(db: &crate::db::DB, user: User) -> Result<(), warp::Rejection> {
        sqlx::query("INSERT INTO user (name, age) VALUES (?, ?)")
            .bind(user.name)
            .bind(user.age)
            .execute(db)
            .await
            .unwrap();
        Ok(())
    }

    pub async fn list_user(db: &crate::db::DB) -> Result<Vec<User>, warp::Rejection> {
        let users: Vec<User> = sqlx::query_as("SELECT * FROM user")
            .fetch_all(db)
            .await
            .unwrap();

        Ok(users)
    }
}
