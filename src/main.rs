use actix_files as fs;
use actix_web::{web, App, HttpServer, Result};

async fn index() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("web/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", "web").show_files_listing())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:3030")?
    .run()
    .await
}
