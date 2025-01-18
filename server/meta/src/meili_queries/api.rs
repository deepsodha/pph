use super::helper::search;
use actix_web::{web, HttpResponse};
use common::dto::{
    doctor_details::DoctorDetails, drugs::Drugs, icd::ICD10, location::Location,
    search_query::SearchQuery,
};

pub async fn doctor_search(query: web::Json<SearchQuery>) -> HttpResponse {
    search::<DoctorDetails>("doctors", query).await
}

pub async fn drug_search(query: web::Json<SearchQuery>) -> HttpResponse {
    search::<Drugs>("drugs", query).await
}

pub async fn location_search(query: web::Json<SearchQuery>) -> HttpResponse {
    search::<Location>("location", query).await
}

pub async fn icd_search(query: web::Json<SearchQuery>) -> HttpResponse {
    search::<ICD10>("icd", query).await
}
