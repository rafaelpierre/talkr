use actix_files as fs;
use actix_web::{web, App, HttpServer, Result};
extern crate pretty_env_logger;
#[macro_use] extern crate log;
use std::env;

async fn index() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("web/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env::set_var("RUST_LOG", "INFO");
    pretty_env_logger::init();
    info!("Starting talkr");

    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", "web").show_files_listing())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:3030")?
    .run()
    .await
}
