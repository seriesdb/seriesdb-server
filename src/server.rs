use crate::handler::Handler;
use crate::settings::SETTINGS;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

pub fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(Handler::new(), &req, stream)
}

pub fn run() {
    let sys = actix::System::new("seriesdb_server");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
    })
    .bind(format!("{}:{}", "0.0.0.0", SETTINGS.server_port))
    .unwrap()
    .start();
    log::info!("Started server successfully!");
    let _ = sys.run();
}
