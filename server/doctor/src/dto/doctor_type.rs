use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, Default, PartialEq, Deserialize, TS, Copy, Eq, sqlx::Type)]
#[ts(export, export_to = "../../bindings/DoctorType.ts")]
pub enum DoctorType {
    #[default]
    FamilyMedicinePhysician,
    Pediatrician,
    Gynecologist,
    Cardiologist,
    Pharmacist,
    Dermatologist,
    Psychiatrist,
    Surgeon,
}

impl DoctorType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DoctorType::FamilyMedicinePhysician => "FamilyMedicinePhysician",
            DoctorType::Pediatrician => "Pediatrician",
            DoctorType::Gynecologist => "Gynecologist",
            DoctorType::Cardiologist => "Cardiologist",
            DoctorType::Pharmacist => "Pharmacist",
            DoctorType::Dermatologist => "Dermatologist",
            DoctorType::Psychiatrist => "Psychiatrist",
            DoctorType::Surgeon => "Surgeon",
        }
    }
}
