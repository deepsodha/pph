use chrono::{DateTime, Utc};
use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{appointment_choose_type::ChooseAppointmentType, appointment_visit_type::VisitType};

#[derive(Deserialize, Serialize, Debug, Default, Clone, validator::Validate, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/AppointmentAdd.ts")]
pub struct AppointmentAdd {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub patient_id: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub doctor_id: String,
    pub doctor_name: Option<String>,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub patient_name: String,
    pub visit: VisitType,
    pub date: DateTime<Utc>,
    #[validate(range(min = 10, max = 60, message = "field can't be empty"))]
    pub appointment_duration: u32,
    pub choose_appointment: ChooseAppointmentType,
    #[validate(length(min = 1, max = 250, message = "field can't be empty"))]
    pub note: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub room_and_equipment_no: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub staff_id: String,
    pub staff_name: String,
    pub last_updated_input: LastUpdatedInput,
}
