use actix_web::*;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(std::fs::read_to_string("web/index.html").unwrap())
}

#[get("/{file}")]
async fn file(path: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(std::fs::read(format!("web/{}", path.into_inner())).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(file))
        .bind(("localhost", 5000))?
        .run()
        .await
}
