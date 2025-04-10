const TABLE_FILE: &str = "sql/table.sql";
const DB_HOST: &str = "localhost:3306";
const DB_USER: &str = "root";
const DB_PWD: &str = "12345678";
const DB_NAME: &str = "warp_sqlx";
const DB_MAX_CONN: u32 = 10;

pub type DB = sqlx::Pool<sqlx::mysql::MySql>;

pub async fn init_db() -> Result<std::sync::Arc<DB>, Box<dyn std::error::Error>> {
    {
        let db = new_db(DB_HOST, DB_USER, DB_PWD, DB_NAME, 1).await?;
        init_table(&db, TABLE_FILE).await?;
    }

    let db = new_db(DB_HOST, DB_USER, DB_PWD, DB_NAME, DB_MAX_CONN).await?;
    Ok(std::sync::Arc::new(db))
}

async fn new_db(
    host: &str,
    user: &str,
    pwd: &str,
    db_name: &str,
    max_conn: u32,
) -> Result<DB, Box<dyn std::error::Error>> {
    let db = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(max_conn)
        .acquire_timeout(std::time::Duration::from_millis(500))
        .connect(&format!("mysql://{}:{}@{}/{}", user, pwd, host, db_name))
        .await?;

    Ok(db)
}

async fn init_table(db: &DB, file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(file)?;
    let sqls = content.split(";");
    for sql in sqls {
        if sql.trim().is_empty() {
            continue;
        }

        match sqlx::query(sql).fetch_all(db).await {
            Ok(_) => (),
            Err(e) => {
                println!("sqlx::query error:{}", e);
                return Err(Box::new(e));
            }
        }
    }

    Ok(())
}
