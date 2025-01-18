use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, Default, PartialEq, Deserialize, TS, Copy, Eq, sqlx::Type)]
#[ts(export, export_to = "../../bindings/StaffType.ts")]
pub enum StaffType {
    #[default]
    Consultation,
    Examination,
    Diagnosis,
    Treatment,
    Procedure,
    Surgery,
    Therapy,
    Counseling,
    MedicationManagement,
    Imaging,
    LaboratoryTest,
    Rehabilitation,
    HomeCare,
    Telemedicine,
    WellnessProgram,
}

impl StaffType {
    pub fn as_str(&self) -> &'static str {
        match self {
            StaffType::Consultation => "Consultation",
            StaffType::Examination => "Examination",
            StaffType::Diagnosis => "Diagnosis",
            StaffType::Treatment => "Treatment",
            StaffType::Procedure => "Procedure",
            StaffType::Surgery => "Surgery",
            StaffType::Therapy => "Therapy",
            StaffType::Counseling => "Counseling",
            StaffType::MedicationManagement => "MedicationManagement",
            StaffType::Imaging => "Imaging",
            StaffType::LaboratoryTest => "LaboratoryTest",
            StaffType::Rehabilitation => "Rehabilitation",
            StaffType::HomeCare => "HomeCare",
            StaffType::Telemedicine => "Telemedicine",
            StaffType::WellnessProgram => "WellnessProgram",
        }
    }
}
