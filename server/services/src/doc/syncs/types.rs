use appointment::domain::appointment_domain::AppointmentState;
use chrono::{DateTime, Utc};
use doctor::domain::doctor_domain::DoctorState;
use organization::domain::organization_domain::OrganizationState;
use patient::domain::{
    allergies::allergies_domain::AllergiesState,
    history::{
        familyhistory::familyhistory_domain::FamilyHistoryState,
        hospitalization::hospitalization_domain::HospitalizationState,
        implantabledevices::implantable_devices_domain::ImplantableDevicesState,
        obandpregnancy::obandpregnancy_domain::OBandPregnancyState,
        pastmedical_history::pastmedical_history_domain::PastMedicalHistoryState,
        pastsurgical_history::pastsurgical_history_domain::PastSurgicalHistoryState,
        social_history::social_history_domain::SocialHistoryState,
    },
    immunizations::{
        addhistorical::addhistorical_domain::HistoricalState,
        administer::administer_domain::AdministerState,
        notadministered::notadministered_domain::NotAdministeredState,
        order::order_domain::OrderState,
    },
    medication::medication_domain::MedicationsState,
    notes::note_domain::NoteState,
    patient_domain::PatientState,
    problem::problem_domain::ProblemState,
    vital::vital_domain::VitalsState,
};
use prescription::domain::prescription_domain::PrescriptionState;
use serde::{Deserialize, Serialize};
use servicelocation::domain::service_location_domain::ServiceLocationState;
use staff::domain::staff_domain::StaffState;
use system_admin::domain::systemadmin_domain::SystemAdminState;
use ts_rs::TS;
use user::domain::user_domain::UserState;

#[derive(Clone, Debug, Default, Serialize, PartialEq, Eq, TS, Deserialize)]
#[ts(export, export_to = "../../bindings/SyncData.ts")]
pub struct SyncData {
    pub system_admin: Vec<SystemAdminState>,
    pub doctors: Vec<DoctorState>,
    pub patients: Vec<PatientState>,
    pub appointments: Vec<AppointmentState>,
    pub prescription: Vec<PrescriptionState>,
    pub service_location: Vec<ServiceLocationState>,
    pub add_historical: Vec<HistoricalState>,
    pub administer: Vec<AdministerState>,
    pub allergy: Vec<AllergiesState>,
    pub medication: Vec<MedicationsState>,
    pub not_administer: Vec<NotAdministeredState>,
    pub order: Vec<OrderState>,
    pub problems: Vec<ProblemState>,
    pub vitals: Vec<VitalsState>,
    pub familyhistory: Vec<FamilyHistoryState>,
    pub hospitalization: Vec<HospitalizationState>,
    pub implantabledevices: Vec<ImplantableDevicesState>,
    pub obandpregnancy: Vec<OBandPregnancyState>,
    pub pastmedicalhistory: Vec<PastMedicalHistoryState>,
    pub pastsurgicalhistory: Vec<PastSurgicalHistoryState>,
    pub socialhistory: Vec<SocialHistoryState>,
    pub staff: Vec<StaffState>,
    pub note: Vec<NoteState>,
    pub user: Vec<UserState>,
    pub organization: Vec<OrganizationState>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct DataTable {
    pub id: String,
    pub org_id: String,
    pub stream_id: String,
    pub version: u32,
    pub data: serde_json::Value,
    pub last_updated: DateTime<Utc>,
}
