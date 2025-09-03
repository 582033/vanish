use actix_governor::{Governor, GovernorConfigBuilder};
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use vanish::{create_secret, get_secret, AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 从环境变量读取端口，如果未设置则使用默认值 5820
    let port = env::var("APP_PORT")
        .unwrap_or_else(|_| "5820".to_string())
        .parse::<u16>()
        .expect("APP_PORT must be a valid port number");

    // 创建应用状态
    let app_data = web::Data::new(AppState {
        secrets: Arc::new(Mutex::new(HashMap::new())),
    });

    // 配置速率限制
    // 允许每分钟20次请求（每3秒1次），并且允许50个请求的突发。
    let governor_conf = GovernorConfigBuilder::default()
        .period(std::time::Duration::from_secs(3))
        .burst_size(50)
        .finish()
        .unwrap();

    println!("Server is running. Access it from your host at http://localhost:{}", port);

    HttpServer::new(move || {
        App::new()
            .wrap(Governor::new(&governor_conf))
            .app_data(app_data.clone())
            .service(create_secret)
            .service(get_secret)
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
