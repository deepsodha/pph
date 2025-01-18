use actix_web::{HttpResponse, Responder};
use serde_json;
use services::doc::organization::helper::read_phonecode_csv;

pub async fn phone_code() -> impl Responder {
    let path = "statics/data/country_and_phone_code.csv";
    match read_phonecode_csv(path) {
        Ok(records) => {
            let json_string = match serde_json::to_string(&records) {
                Ok(json) => json,
                Err(_) => return HttpResponse::InternalServerError().finish(),
            };

            HttpResponse::Ok()
                .content_type("application/json")
                .body(json_string)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
