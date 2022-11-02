use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use deadpool_postgres::{Config, Pool};
use serde::{Deserialize, Serialize};
use tokio_postgres::NoTls;

#[derive(Serialize, Deserialize)]
struct Character {
    id: i32,
    name: String,
    age: i32,
    alive: bool,
}

async fn handler(pool: web::Data<Pool>) -> impl Responder {
    let client = pool.get().await.unwrap();
    let stmt = client
        .prepare("SELECT id, name, age, alive FROM people")
        .await
        .unwrap();
    let _rows = client.query(&stmt, &[]).await.unwrap();
    let mut characters = Vec::new();
    for row in _rows {
        let character = Character {
            id: row.get(0),
            name: row.get(1),
            age: row.get(2),
            alive: row.get(3),
        };
        characters.push(character);
    }
    HttpResponse::Ok().json(characters)

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut pool = Config::new();
    pool.host = Some("localhost".to_string());
    pool.port = Some(5432);
    pool.dbname = Some("postgres".to_string());
    pool.user = Some("postgres".to_string());
    pool.password = Some("postgres".to_string());
    let pool = pool.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(handler))
    })
        .bind(("127.0.0.1", 8080))?
        .run();
    println!("Server running at http://{}/", "127.0.0.1");

    server.await
}
