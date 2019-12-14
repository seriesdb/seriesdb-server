use crate::db_owner::DbOwner;
use crate::handler::Handler;
use crate::settings::SETTINGS;
use actix::Addr;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

pub fn index(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<Addr<DbOwner>>,
) -> Result<HttpResponse, Error> {
    ws::start(Handler::new(state.get_ref().clone()), &req, stream)
}

pub fn start(db_owner_addr: Addr<DbOwner>) {
    HttpServer::new(move || {
        App::new()
            .data(db_owner_addr.clone())
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
    })
    .bind(format!("{}:{}", "0.0.0.0", SETTINGS.server_port))
    .unwrap()
    .start();
    log::info!("Started server successfully!");
}
