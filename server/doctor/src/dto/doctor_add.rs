use super::doctor_type::DoctorType;
use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/DoctorAdd.ts")]
pub struct DoctorAdd {
    pub user_id: String,
    pub doctor_role: Vec<DoctorType>,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub docter_register_number: String,
    pub doctor_department: String,
    pub doctor_speciality: String,
    pub emergency: bool,
    pub last_updated_input: LastUpdatedInput,
}
