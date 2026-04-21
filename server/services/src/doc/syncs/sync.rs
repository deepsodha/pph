use super::{helper::fetch_states, types::SyncData};
use anyhow::Result;
use appointment::domain::appointment_domain::AppointmentState;
use common::dto::last_updated_input::LastUpdatedInput;
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
use servicelocation::domain::service_location_domain::ServiceLocationState;
use sqlx::{Pool, Sqlite};
use staff::domain::staff_domain::StaffState;
use system_admin::domain::systemadmin_domain::SystemAdminState;
use user::domain::user_domain::UserState;

pub async fn sync(
    input: LastUpdatedInput,
    org_id: String,
    state: Pool<Sqlite>,
) -> Result<SyncData> {
    let limit = input.limit;
    let patient_states = fetch_states::<PatientState>(
        "patient_table_state",
        &input.patients,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;
    let systemadmin_states = fetch_states::<SystemAdminState>(
        "systemadmin_table_state",
        &input.system_admin,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;
    let doctor_states = fetch_states::<DoctorState>(
        "doctor_table_state",
        &input.doctors,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;
    let appointment_states = fetch_states::<AppointmentState>(
        "appointment_table_state",
        &input.appointments,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;
    let prescription_states = fetch_states::<PrescriptionState>(
        "prescription_table_state",
        &input.prescription,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;
    let servicelocation_states = fetch_states::<ServiceLocationState>(
        "servicelocation_table_state",
        &input.service_location,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;
    let addhistorical_states = fetch_states::<HistoricalState>(
        "addhistorical_table_state",
        &input.add_historical,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;
    let administer_states = fetch_states::<AdministerState>(
        "administer_table_state",
        &input.administer,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;
    let allergy_states = fetch_states::<AllergiesState>(
        "allergies_table_state",
        &input.allergy,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;
    let medication_states = fetch_states::<MedicationsState>(
        "medication_table_state",
        &input.medication,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;
    let notadminister_states = fetch_states::<NotAdministeredState>(
        "notadminister_table_state",
        &input.not_administer,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;
    let order_states = fetch_states::<OrderState>(
        "order_table_state",
        &input.order,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;
    let problem_states = fetch_states::<ProblemState>(
        "problem_table_state",
        &input.problems,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;
    let vitals_states = fetch_states::<VitalsState>(
        "vitals_table_state",
        &input.vitals,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;
    let familyhistory_states = fetch_states::<FamilyHistoryState>(
        "familyhistory_table_state",
        &input.familyhistory,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;
    let hospitalization_states = fetch_states::<HospitalizationState>(
        "hospitalization_table_state",
        &input.hospitalization,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;
    let implantabledevices_states = fetch_states::<ImplantableDevicesState>(
        "implantabledevices_table_state",
        &input.implantabledevices,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;
    let obandpregnanacy_states = fetch_states::<OBandPregnancyState>(
        "obandpregnancy_table_state",
        &input.obandpregnanacy,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;
    let pastmedicalhistory_states = fetch_states::<PastMedicalHistoryState>(
        "pastmedicalhistory_table_state",
        &input.pastmedicalhistory,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;
    let pastsurgicalhistory_states = fetch_states::<PastSurgicalHistoryState>(
        "pastsurgicalhistory_table_state",
        &input.pastsurgicalhistory,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;
    let socialhistory_states = fetch_states::<SocialHistoryState>(
        "socialhistory_table_state",
        &input.socialhistory,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;
    let note_states = fetch_states::<NoteState>(
        "note_table_state",
        &input.note,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;
    let staff_states = fetch_states::<StaffState>(
        "staff_table_state",
        &input.staff,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;
    let user_states = fetch_states::<UserState>(
        "user_table_state",
        &input.user,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;
    let organization_states = fetch_states::<OrganizationState>(
        "organization_table_state",
        &input.organization,
        &org_id,
        limit,
        state.clone(),
    )
    .await?;

    let data = SyncData {
        system_admin: systemadmin_states,
        doctors: doctor_states,
        patients: patient_states,
        appointments: appointment_states,
        prescription: prescription_states,
        service_location: servicelocation_states,
        add_historical: addhistorical_states,
        administer: administer_states,
        allergy: allergy_states,
        medication: medication_states,
        not_administer: notadminister_states,
        order: order_states,
        problems: problem_states,
        vitals: vitals_states,
        familyhistory: familyhistory_states,
        hospitalization: hospitalization_states,
        implantabledevices: implantabledevices_states,
        obandpregnancy: obandpregnanacy_states,
        pastmedicalhistory: pastmedicalhistory_states,
        pastsurgicalhistory: pastsurgicalhistory_states,
        socialhistory: socialhistory_states,
        staff: staff_states,
        note: note_states,
        user: user_states,
        organization: organization_states,
    };
    Ok(data)
}
