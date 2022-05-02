use actix_web::web::{Data, Json, Query};
use actix_web::{get, post};
use actix_web::{HttpResponse, Responder};
use log::debug;

use crate::{db, Server};

#[post("/csv")]
async fn handle_post_csv(server: Data<Server>) -> impl Responder {
    let logs = Default::default();

    HttpResponse::Ok().json(api::csv::post::Response(logs))
}

#[post("/logs")]
async fn handle_post_logs(
    server: Data<Server>,
    log: Json<api::logs::post::Request>,
) -> impl Responder {
    use crate::model::NewLog;
    use chrono::Utc;

    let log = NewLog {
        user_agent: log.user_agent.clone(),
        response_time: log.response_time,
        timestamp: log.timestamp.unwrap_or_else(|| Utc::now()).naive_utc(),
    };
    let conn = server.pool.get().map_err(|err| debug!("Error: {:?}", err));
    db::insert_log(&conn.unwrap(), &log).map_err(|err| debug!("Error: {:?}", err));

    debug!("received log: {:?}", log);

    HttpResponse::Accepted().finish()
}

#[get("/logs")]
async fn handle_get_logs(
    server: Data<Server>,
    range: Query<api::logs::get::Query>,
) -> impl Responder {
    use chrono::{DateTime, Utc};

    let conn = server.pool.get();
    let logs = db::logs(&conn.unwrap(), range.from, range.until).unwrap();
    let logs = logs
        .into_iter()
        .map(|log| api::Log {
            user_agent: log.user_agent,
            response_time: log.response_time,
            timestamp: DateTime::from_utc(log.timestamp, Utc),
        })
        .collect();

    HttpResponse::Ok().json(api::logs::get::Response(logs))
}

/// GET /csvのハンドラ
#[get("/csv")]
async fn handle_get_csv(
    server: Data<Server>,
    range: Query<api::csv::get::Query>,
) -> impl Responder {
    debug!("{:?}", range);
    let csv: Vec<u8> = vec![];
    HttpResponse::Ok()
        .append_header(("Content-Type", "text/csv"))
        .body(csv)
}
