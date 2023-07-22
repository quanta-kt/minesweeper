mod create_game;
mod join_game;
mod ws;

use std::path::PathBuf;

use actix_files::{Files, NamedFile};
use actix_web::{web, HttpRequest};

async fn index(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let static_dir: PathBuf = "./static/".parse().unwrap();
    let filename: PathBuf = req.match_info().query("filename").parse().unwrap();

    let path = static_dir.join(filename);

    if path == static_dir {
        Ok(NamedFile::open(static_dir.join("index.html"))?)
    } else {
        Ok(NamedFile::open(static_dir.join(path))?)
    }
}

pub fn routes() -> actix_web::Scope {
    let api_service = web::scope("/api")
        .service(create_game::create_game)
        .service(join_game::join_game);

    web::scope("")
        .service(api_service)
        .service(Files::new("/", "./static/").index_file("index.html"))
}
