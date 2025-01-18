use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::dto::{
    appointment_choose_type::ChooseAppointmentType, appointment_visit_type::VisitType,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateAppointment {
    pub id: String,
    pub org_id: String,
    pub service_location_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub patient_id: String,
    pub doctor_id: String,
    pub doctor_name: Option<String>,
    pub patient_name: String,
    pub visit: VisitType,
    pub date: DateTime<Utc>,
    pub appointment_duration: u32,
    pub choose_appointment: ChooseAppointmentType,
    pub note: String,
    pub room_and_equipment_no: String,
    pub staff_id: String,
    pub staff_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateAppointment {
    pub id: String,
    pub org_id: String,
    pub service_location_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub patient_id: String,
    pub doctor_id: String,
    pub doctor_name: Option<String>,
    pub patient_name: String,
    pub visit: VisitType,
    pub date: DateTime<Utc>,
    pub appointment_duration: u32,
    pub choose_appointment: ChooseAppointmentType,
    pub note: String,
    pub room_and_equipment_no: String,
    pub staff_id: String,
    pub staff_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteAppointment {
    pub id: String,
    pub patient_id: String,
    pub doctor_id: String,
    pub org_id: String,
    pub service_location_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AppointmentCommands {
    CreateAppointment(CreateAppointment),
    UpdateAppointment(UpdateAppointment),
    DeleteAppointment(DeleteAppointment),
}
