use std::sync::Arc;

type JsonReply = Result<warp::reply::Json, warp::Rejection>;

//get api
pub async fn get_user(db: Arc<crate::db::DB>, id: u32) -> JsonReply {
    let user = crate::model::User::get_user(&db, id).await?;
    Ok(warp::reply::json(&serde_json::json!(user)))
}

pub async fn list_user(db: Arc<crate::db::DB>) -> JsonReply {
    let users = crate::model::User::list_user(&db).await?;
    Ok(warp::reply::json(&serde_json::json!(users)))
}

//create api
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateUserReq {
    pub name: String,
    pub age: u32,
}

pub async fn create_user(db: Arc<crate::db::DB>, req: CreateUserReq) -> JsonReply {
    let user = crate::model::User {
        id: 0,
        name: req.name,
        age: req.age,
        ctime: chrono::Local::now().into(),
    };

    match crate::model::User::create_user(&db, user).await {
        Ok(_) => Ok(warp::reply::json(&serde_json::json!("OK"))),
        Err(e) => Err(e),
    }
}

//update api
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UpdateUserReq {
    pub id: u32,
    pub name: String,
    pub age: u32,
}
pub async fn update_user(db: Arc<crate::db::DB>, req: UpdateUserReq) -> JsonReply {
    Ok(warp::reply::json(&serde_json::json!("hello world")))
}

//delte api
pub async fn delete_user(db: Arc<crate::db::DB>, id: u32) -> JsonReply {
    Ok(warp::reply::json(&serde_json::json!("hello world")))
}
