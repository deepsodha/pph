use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "../../bindings/DoctorDetails.ts")]
pub struct DoctorDetails {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "DoctorName")]
    pub doctor_name: String,
    #[serde(rename = "Speciality")]
    pub speciality: String,
    #[serde(rename = "Experience")]
    pub experience: String,
    #[serde(rename = "HospitalAddress")]
    pub hospital_address: String,
    #[serde(rename = "City")]
    pub city: String,
    #[serde(rename = "Pincode")]
    pub pin_code: i64,
}
