use crate::doc::disease_prediction::SymptomsNames;
use crate::doc::medicine_prediction::DiseaseNames;
use crate::method::convention::{ErrorData, Request};
use crate::rpc::create_request;
use crate::rpc::Rpc;
use common::dto::last_updated_input::LastUpdatedInput;
use db_services::tables::organization::Getorganization;
use organization::dto::csv::PinCodeInput;
use organization::dto::organization_add::OrganizationAdd;
use organization::dto::organization_by_id::OrganizationsByID;
use organization::dto::organization_delete::OrganizationDelete;
use organization::dto::organization_update::OrganizationUpdate;
use organization::dto::select_organization::SelectOrganization;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum OrganizationMethods {
    Add(OrganizationAdd, Option<String>),
    Update(OrganizationUpdate, Option<String>),
    Select(SelectOrganization, Option<String>),
    Delete(OrganizationDelete, Option<String>),
    Get(OrganizationsByID, Option<Getorganization>),
    GetAll,
    Vaccines,
    Drugs,
    Location(PinCodeInput, Option<String>),
    PinCodes,
    Sync(LastUpdatedInput, Option<String>),
    MedicinePrediction(DiseaseNames),
    DiseasePrediction(SymptomsNames),
}
impl Rpc for OrganizationMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        //Add as str
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<OrganizationAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(OrganizationMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<OrganizationUpdate>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(OrganizationMethods::Update(input, None))
                }
                "Select" => {
                    let input = serde_json::from_value::<SelectOrganization>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(OrganizationMethods::Select(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<OrganizationDelete>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(OrganizationMethods::Delete(input, None))
                }
                "Get" => {
                    let input = serde_json::from_value::<OrganizationsByID>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(OrganizationMethods::Get(input, None))
                }
                "GetAll" => Ok(OrganizationMethods::GetAll),
                "MedicinePrediction" => {
                    let input = serde_json::from_value::<DiseaseNames>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(OrganizationMethods::MedicinePrediction(input))
                }
                "DiseasePrediction" => {
                    let input = serde_json::from_value::<SymptomsNames>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(OrganizationMethods::DiseasePrediction(input))
                }
                "PinCodes" => Ok(OrganizationMethods::PinCodes),
                "Drugs" => Ok(OrganizationMethods::Drugs),
                "Vaccines" => Ok(OrganizationMethods::Vaccines),
                "Location" => {
                    let input = serde_json::from_value::<PinCodeInput>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(OrganizationMethods::Location(input, None))
                }
                "Sync" => {
                    let input = serde_json::from_value::<LastUpdatedInput>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(OrganizationMethods::Sync(input, None))
                }
                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };
        res
    }
    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        match self {
            OrganizationMethods::Add(input, _) => create_request(namespace, "Add", input),

            OrganizationMethods::Update(input, _) => create_request(namespace, "Update", input),

            OrganizationMethods::Select(input, _) => create_request(namespace, "Select", input),

            OrganizationMethods::Delete(input, _) => create_request(namespace, "Delete", input),

            OrganizationMethods::Get(input, _) => create_request(namespace, "Get", input),
            OrganizationMethods::MedicinePrediction(input) => {
                create_request(namespace, "MedicinePrediction", input)
            }
            OrganizationMethods::DiseasePrediction(input) => {
                create_request(namespace, "DiseasePrediction", input)
            }
            OrganizationMethods::GetAll => create_request(namespace, "GetAll", Value::Null),
            OrganizationMethods::PinCodes => create_request(namespace, "PinCodes", Value::Null),
            OrganizationMethods::Drugs => create_request(namespace, "Drugs", Value::Null),
            OrganizationMethods::Vaccines => create_request(namespace, "Vaccines", Value::Null),
            OrganizationMethods::Location(input, _) => create_request(namespace, "Location", input),
            OrganizationMethods::Sync(input, _) => create_request(namespace, "Sync", input),
        }
    }
}
