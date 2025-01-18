use actix_web::{HttpResponse, Responder};
use ml::medicine_prediction::get_medicine_prediction;

pub async fn medicine_prediction() -> impl Responder {
    let user_input = vec!["Asthma".to_string(), "Allergic rhinitis".to_string()];

    get_medicine_prediction(user_input);

    HttpResponse::Ok().body("Hey there, coming from medicine prediction")
}
