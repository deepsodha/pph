use crate::{
    app_state::AppState,
    doc::patients::notes::{
        accupuncture_followup::add_acupuncturefollowup, amendment::add_amendment, group::add_group,
        historyandphysical::add_historyandphysical, medspaprocedure::add_medspaprocedure,
        officeform::add_officeform, phone::add_phone, soap::add_soap,
    },
    get_organization_id::organization_id_fetch,
    method::{
        convention::{self, ErrorData},
        methods::gp_methods::patients::notes::{
            acupuncture_follow_up::AcupunctureFollowUpMethods, amendment::AmendmentMethods,
            group::GroupMethods, history_and_physical::HistoryAndPhysicalMethods,
            med_spa_procedure::MedSpaProcedureMethods, note::NotesMethods,
            office_form::OfficeFormMethods, phone::PhoneMethods, soap::SoapMethods,
        },
    },
};

use serde_json::Value;
use utils::biscuit::check_role::check_role_exist;

pub async fn process(
    methods: NotesMethods,
    state: AppState,
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
                (true, NotesMethods::Soap(boxed_method)) => match *boxed_method {
                    SoapMethods::Add(input, _) => {
                        add_soap(input, state.clone(), org_id, user_id).await
                    }
                },

                (true, NotesMethods::HistoryAndPhysical(boxed_method)) => match *boxed_method {
                    HistoryAndPhysicalMethods::Add(input, _) => {
                        add_historyandphysical(input, state.clone(), org_id, user_id).await
                    }
                },

                (true, NotesMethods::AcupunctureFollowUp(boxed_method)) => match *boxed_method {
                    AcupunctureFollowUpMethods::Add(input, _) => {
                        add_acupuncturefollowup(input, state.clone(), org_id, user_id).await
                    }
                },

                (true, NotesMethods::Amendment(boxed_method)) => match *boxed_method {
                    AmendmentMethods::Add(input, _) => {
                        add_amendment(input, state.clone(), org_id, user_id).await
                    }
                },
                (true, NotesMethods::Group(boxed_method)) => match *boxed_method {
                    GroupMethods::Add(input, _) => {
                        add_group(input, state.clone(), org_id, user_id).await
                    }
                },
                (true, NotesMethods::MedSpaProcedure(boxed_method)) => match *boxed_method {
                    MedSpaProcedureMethods::Add(input, _) => {
                        add_medspaprocedure(input, state.clone(), org_id, user_id).await
                    }
                },
                (true, NotesMethods::Phone(boxed_method)) => match *boxed_method {
                    PhoneMethods::Add(input, _) => {
                        add_phone(input, state.clone(), org_id, user_id).await
                    }
                },
                (true, NotesMethods::OfficeForm(boxed_method)) => match *boxed_method {
                    OfficeFormMethods::Add(input, _) => {
                        add_officeform(input, state.clone(), org_id, user_id).await
                    }
                },
                (false, _) => Err(convention::ErrorData {
                    code: -32600,
                    message: String::from("Access Denied"),
                    data: Value::Null,
                }),
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
