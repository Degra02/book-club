use std::{io, path::{Path, PathBuf}};
use rocket::fs::NamedFile;


#[get("/")]
pub async fn index() -> io::Result<NamedFile>{
    let page_directory_path = format!("{}/frontend/build", env!("CARGO_MANIFEST_DIR"));
    NamedFile::open(Path::new(&page_directory_path).join("index.html")).await
}

#[get("/<file..>")]
pub async fn files(file: PathBuf) -> io::Result<NamedFile> {
    let page_directory_path = 
  format!("{}/frontend/build", env!("CARGO_MANIFEST_DIR"));
  NamedFile::open(Path::new(&page_directory_path).join(file)).await
}
