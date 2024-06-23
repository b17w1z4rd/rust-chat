use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyData {
    name: String,
    age: u32,
}

async fn greet(data: web::Json<MyData>) -> impl Responder {
    format!("Hello {}! You are {} years old.", data.name, data.age)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/greet", web::post().to(greet))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
