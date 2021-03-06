use tokio::sync::Mutex;
use serde::Deserialize;
use actix_web::rt::net::TcpStream;
use actix_web::{web, post, HttpResponse};
use crate::app::AppError;
use crate::app::helpers::initialization;


#[derive(Deserialize)]
pub struct DtcChangeUnit {
    lrn: u8,
    unx: u8
}

#[post("/changeunit")]
pub async fn set_unit(
    payload: web::Json<DtcChangeUnit>,
    tcp: web::Data<Mutex<TcpStream>>
) -> Result<HttpResponse, AppError> {
    let buffer = [0u8; 8];
    let mut stream = tcp.lock().await;
    let stream = &mut *stream;

    let message = initialization::change_unit(
        stream,
        buffer,
        payload.0.lrn,
        payload.0.unx
    ).await?;

    Ok(HttpResponse::Ok().json(message))
}
