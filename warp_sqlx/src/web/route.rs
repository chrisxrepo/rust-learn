use std::sync::Arc;
use warp::Filter;

pub fn init_route(
    base_path: &str,
    db: Arc<crate::db::DB>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let list_user = warp::path(base_path)
        .and(warp::path("users"))
        .and(warp::get())
        .and(crate::web::with_db(db.clone()))
        .and_then(crate::web::user::list_user);

    let base_path = warp::path(base_path).and(warp::path("user"));

    let get_user = base_path
        .and(warp::get())
        .and(crate::web::with_db(db.clone()))
        .and(warp::path::param())
        .and_then(crate::web::user::get_user);

    let create_user = base_path
        .and(warp::post())
        .and(crate::web::with_db(db.clone()))
        .and(warp::body::json())
        .and_then(crate::web::user::create_user);

    let update_user = base_path
        .and(warp::put())
        .and(crate::web::with_db(db.clone()))
        .and(warp::body::json())
        .and_then(crate::web::user::update_user);

    let delete_user = base_path
        .and(warp::delete())
        .and(crate::web::with_db(db.clone()))
        .and(warp::path::param())
        .and_then(crate::web::user::delete_user);

    list_user
        .or(get_user)
        .or(create_user)
        .or(update_user)
        .or(delete_user)
}
