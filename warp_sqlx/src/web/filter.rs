use std::{convert::Infallible, sync::Arc};
use warp::Filter;

pub fn with_db(
    db_pool: Arc<crate::db::DB>,
) -> impl Filter<Extract = (Arc<crate::db::DB>,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
