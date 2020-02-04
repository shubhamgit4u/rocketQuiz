#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use rocket::response::NamedFile;
extern crate diesel;
#[macro_use] extern crate serde_derive;
use rocket_contrib::templates::Template;
use std::path::{Path, PathBuf};

#[derive(Serialize)]
struct TemplateContext {
    items: Vec<&'static str>,
}
#[get("/")]
fn index() -> Template {
    let context = TemplateContext {
        items: vec!["input", "from","backend"]
    };
    Template::render("index", &context)
}

#[get("/login")]
fn log() -> Template {
    let context = TemplateContext {
        items: vec!["input", "from","backend"]
    };
    Template::render("login", &context)
}
#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}
fn rock() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, log, files])
        .attach(Template::fairing())
}
fn main() {
    rock().launch();
}
