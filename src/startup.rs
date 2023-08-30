use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::{Pool, Postgres};

use crate::routes;

pub fn run(listener: TcpListener, pool: Pool<Postgres>) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let connection = web::Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
