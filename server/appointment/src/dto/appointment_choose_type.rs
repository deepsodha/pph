use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, Default, PartialEq, Deserialize, TS, Copy, Eq, sqlx::Type)]
#[ts(export, export_to = "../../bindings/ChooseAppointmentType.ts")]
pub enum ChooseAppointmentType {
    #[default]
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
}
impl ChooseAppointmentType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ChooseAppointmentType::Weekly => "Weekly",
            ChooseAppointmentType::Monthly => "Monthly",
            ChooseAppointmentType::Quarterly => "Quarterly",
            ChooseAppointmentType::Yearly => "Yearly",
        }
    }
}
