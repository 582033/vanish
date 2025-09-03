use actix_files::Files;
use actix_web::{App, HttpServer, web};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use vanish::{AppState, create_secret, get_secret};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 创建应用状态
    let app_data = web::Data::new(AppState {
        secrets: Arc::new(Mutex::new(HashMap::new())),
    });

    println!("Server is running on http://127.0.0.1:5820");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(create_secret)
            .service(get_secret)
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("127.0.0.1", 5820))?
    .run()
    .await
}
