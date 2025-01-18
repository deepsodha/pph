use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "../../bindings/Doctors.ts")]
pub struct Doctors {
    pub doctor_name: String,
    pub speciality: String,
    pub experience: String,
    pub hospital_address: String,
    pub city: String,
    pub pincode: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "../../bindings/GetDoctors.ts")]
pub struct GetDoctors {
    pub id: String,
    pub doctor_name: String,
    pub speciality: String,
    pub experience: String,
    pub hospital_address: String,
    pub city: String,
    pub pincode: String,
    pub is_deleted: bool,
}
