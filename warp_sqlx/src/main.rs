mod db;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe { std::env::set_var("RUST_APP_LOG", "debug") }
    pretty_env_logger::init_custom_env("RUST_APP_LOG");

    let db = db::init_db().await?;
    let apis = web::init_route("api", db);

    let addr: std::net::SocketAddr = "0.0.0.0:8000".parse().unwrap();
    warp::serve(apis).run(addr).await;
    Ok(())
}
