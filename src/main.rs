use actix_files::NamedFile;
use actix_web::App;
use actix_web::http::header::ContentDisposition;
use actix_web::http::header::DispositionType;
use actix_web::web::Data;
use actix_web::{HttpServer, get};
use file_downloader::db::get_member;
use file_downloader::db::init_db;
use file_downloader::db::store_member;
use file_downloader::downloader::Member;
use file_downloader::downloader::create_csv;
use sqlx::PgPool;

use std::env;
use std::io;
use dotenv;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    dotenv::dotenv().ok();

    let pool = init_db().await;

    let members = Member::members();

    // Populate Database
    for member in members.into_iter() {
        store_member(member, &pool);
    }

    let pool_data = Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .service(download_members_csv)
            .app_data(pool_data.clone()) // Add database pool as an app data
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[get("/download-csv")]
pub async fn download_members_csv(
    pool: Data<PgPool>,
) -> actix_web::Result<NamedFile> {
    let members = get_member(&pool).await;
    let path = env::var("MEMBERS_CSV_PATH").unwrap_or_else(|_| "".to_owned());
    create_csv(members, path);
    let file = NamedFile::open(path)?;

    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        })
    )
}


