use super::{history_services, notes_services};
use crate::app_state::AppState;
use crate::doc::patients::addhistorical::command::{
    add_historical, delete_historical, update_historical,
};
use crate::doc::patients::addminister::command::{
    add_administer, delete_administer, update_administer,
};
use crate::doc::patients::allergy::command::{
    add_allergy, delete_allergy, get_all_allergies_handler, update_allergy,
};
use crate::doc::patients::command::{add_patient, delete_patient, update_patient};
use crate::doc::patients::medication::command::{
    add_medication, delete_medication, update_medication,
};
use crate::doc::patients::notaddminister::command::{
    add_not_administer, delete_not_administer, update_not_administer,
};
use crate::doc::patients::order::command::{add_order, delete_order, update_order};
use crate::doc::patients::problems::commnad::{add_problem, delete_problem, update_problem};
use crate::doc::patients::vitals::command::{add_vitals, delete_vitals, update_vitals};
use crate::get_organization_id::organization_id_fetch;
use crate::method::convention::ErrorData;
use crate::method::methods::gp_methods::patients::allergy::AllergyMethods;
use crate::method::methods::gp_methods::patients::immunization::addhistorical::AddhisitoricalMethods;
use crate::method::methods::gp_methods::patients::immunization::addminister::AddministerMethods;
use crate::method::methods::gp_methods::patients::immunization::notadminister::NotaddministerMethods;
use crate::method::methods::gp_methods::patients::immunization::order::OrderMethods;
use crate::method::methods::gp_methods::patients::medications::MedicationsMethods;
use crate::method::methods::gp_methods::patients::problem::ProblemsMethods;
use crate::method::methods::gp_methods::patients::vitals::VitalsMethods;
use crate::method::{convention, methods::gp_methods::patients::patient::PatientsMethods};
use serde_json::Value;
use utils::biscuit::check_role::check_role_exist;

pub async fn process(
    methods: PatientsMethods,
    state: &AppState,
    org_id: String,
    user_id: String,
    feature_access: bool,
    roles: Vec<String>,
) -> Result<Value, convention::ErrorData> {
    let organization_id = organization_id_fetch(state.clone(), org_id.clone()).await?;

    if organization_id {
        if check_role_exist(
            vec![
                "systemadmin".to_string(),
                "superadmin".to_string(),
                "doctor".to_string(),
                "officestaff".to_string(),
            ],
            &roles,
        ) {
            match (feature_access, methods) {
                (false, _) => Err(convention::ErrorData {
                    code: -32600,
                    message: String::from("Access Denied"),
                    data: Value::Null,
                }),
                (true, PatientsMethods::Add(input)) => {
                    add_patient(input, state.clone(), org_id, user_id).await
                }
                (true, PatientsMethods::Update(input)) => {
                    update_patient(input, state.clone(), org_id, user_id).await
                }
                (true, PatientsMethods::Delete(input)) => {
                    delete_patient(input, state.clone(), org_id, user_id).await
                }
                (true, PatientsMethods::Vitals(VitalsMethods::Add(input, _))) => {
                    add_vitals(input, state.clone(), org_id, user_id).await
                }
                (true, PatientsMethods::Vitals(VitalsMethods::Update(input, _))) => {
                    update_vitals(input, state.clone(), org_id, user_id).await
                }
                (true, PatientsMethods::Vitals(VitalsMethods::Delete(input, _))) => {
                    delete_vitals(input, state.clone(), org_id, user_id).await
                }
                (true, PatientsMethods::Allergy(AllergyMethods::Add(input, _))) => {
                    add_allergy(input, state.clone(), org_id, user_id).await
                }
                (true, PatientsMethods::Allergy(AllergyMethods::Update(input, _))) => {
                    update_allergy(input, state.clone(), org_id, user_id).await
                }
                (true, PatientsMethods::Allergy(AllergyMethods::Delete(input, _))) => {
                    delete_allergy(input, state.clone(), org_id, user_id).await
                }
                (true, PatientsMethods::Allergy(AllergyMethods::Name)) => {
                    get_all_allergies_handler().await
                }
                (true, PatientsMethods::Problems(ProblemsMethods::Add(input, _))) => {
                    add_problem(input, state.clone(), org_id, user_id).await
                }
                (true, PatientsMethods::Problems(ProblemsMethods::Update(input, _))) => {
                    update_problem(input, state.clone(), org_id, user_id).await
                }
                (true, PatientsMethods::Problems(ProblemsMethods::Delete(input, _))) => {
                    delete_problem(input, state.clone(), org_id, user_id).await
                }
                (true, PatientsMethods::Order(OrderMethods::Add(input, _))) => {
                    add_order(input, state.clone(), org_id, user_id).await
                }
                (true, PatientsMethods::Order(OrderMethods::Update(input, _))) => {
                    update_order(input, state.clone(), org_id, user_id).await
                }
                (true, PatientsMethods::Order(OrderMethods::Delete(input, _))) => {
                    delete_order(input, state.clone(), org_id, user_id).await
                }
                (true, PatientsMethods::AddHistorical(AddhisitoricalMethods::Add(input, _))) => {
                    add_historical(input, state.clone(), org_id, user_id).await
                }
                (true, PatientsMethods::AddHistorical(AddhisitoricalMethods::Update(input, _))) => {
                    update_historical(input, state.clone(), org_id, user_id).await
                }
                (true, PatientsMethods::AddHistorical(AddhisitoricalMethods::Delete(input, _))) => {
                    delete_historical(input, state.clone(), org_id, user_id).await
                }
                (true, PatientsMethods::Addminister(AddministerMethods::Add(input, _))) => {
                    add_administer(input, state.clone(), org_id, user_id).await
                }
                (true, PatientsMethods::Addminister(AddministerMethods::Update(input, _))) => {
                    update_administer(input, state.clone(), org_id, user_id).await
                }
                (true, PatientsMethods::Addminister(AddministerMethods::Delete(input, _))) => {
                    delete_administer(input, state.clone(), org_id, user_id).await
                }
                (true, PatientsMethods::Notaddminister(NotaddministerMethods::Add(input, _))) => {
                    add_not_administer(input, state.clone(), org_id, user_id).await
                }
                (
                    true,
                    PatientsMethods::Notaddminister(NotaddministerMethods::Update(input, _)),
                ) => update_not_administer(input, state.clone(), org_id, user_id).await,
                (
                    true,
                    PatientsMethods::Notaddminister(NotaddministerMethods::Delete(input, _)),
                ) => delete_not_administer(input, state.clone(), org_id, user_id).await,
                (true, PatientsMethods::Medications(MedicationsMethods::Add(input, _))) => {
                    add_medication(input, state.clone(), org_id, user_id).await
                }
                (true, PatientsMethods::Medications(MedicationsMethods::Update(input, _))) => {
                    update_medication(input, state.clone(), org_id, user_id).await
                }
                (true, PatientsMethods::Medications(MedicationsMethods::Delete(input, _))) => {
                    delete_medication(input, state.clone(), org_id, user_id).await
                }
                (true, PatientsMethods::History(a)) => {
                    history_services::process(
                        a,
                        state.clone(),
                        org_id,
                        user_id,
                        feature_access,
                        roles,
                    )
                    .await
                }
                (true, PatientsMethods::Notes(a)) => {
                    notes_services::process(
                        a,
                        state.clone(),
                        org_id,
                        user_id,
                        feature_access,
                        roles,
                    )
                    .await
                }
            }
        } else {
            Err(convention::ErrorData {
                code: -32600,
                message: String::from("Insufficient Role"),
                data: Value::Null,
            })
        }
    } else {
        Err(ErrorData {
            message: String::from("ORGANIZATION_NOT_FOUND"),
            data: Value::Null,
            code: -32600,
        })
    }
}
