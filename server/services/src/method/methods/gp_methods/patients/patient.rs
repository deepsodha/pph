use super::allergy::AllergyMethods;
use super::history_page::history::HistoryMethods;
use super::immunization::addhistorical::AddhisitoricalMethods;
use super::immunization::addminister::AddministerMethods;
use super::immunization::notadminister::NotaddministerMethods;
use super::immunization::order::OrderMethods;
use super::medications::MedicationsMethods;
use super::notes::note::NotesMethods;
use super::problem::ProblemsMethods;
use super::vitals::VitalsMethods;
use crate::method::convention::{ErrorData, Request};
use crate::rpc::create_request;
use crate::rpc::Rpc;
use patient::dto::patient::patient_add::PatientAdd;
use patient::dto::patient::patient_delete::PatientDelete;
use patient::dto::patient::patient_update::PatientUpdate;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum PatientsMethods {
    Add(PatientAdd),
    Update(PatientUpdate),
    Delete(PatientDelete),
    Vitals(VitalsMethods),
    Allergy(AllergyMethods),
    Problems(ProblemsMethods),
    Order(OrderMethods),
    AddHistorical(AddhisitoricalMethods),
    Addminister(AddministerMethods),
    Notaddminister(NotaddministerMethods),
    Medications(MedicationsMethods),
    History(HistoryMethods),
    Notes(NotesMethods),
}
impl Rpc for PatientsMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        //GetByID, Problems::GET
        let names = str.split("::").collect::<Vec<_>>();
        let res =
            if let Some((first, elements)) = names.split_first() {
                match *first {
                    "Add" => {
                        let input = serde_json::from_value::<PatientAdd>(data[0].clone())
                            .map_err(|_| ErrorData::std(-32601))?;
                        Ok(PatientsMethods::Add(input))
                    }
                    "Update" => {
                        let input = serde_json::from_value::<PatientUpdate>(data[0].clone())
                            .map_err(|_| ErrorData::std(-32601))?;
                        Ok(PatientsMethods::Update(input))
                    }
                    "Delete" => {
                        let input = serde_json::from_value::<PatientDelete>(data[0].clone())
                            .map_err(|_| ErrorData::std(-32601))?;
                        Ok(PatientsMethods::Delete(input))
                    }
                    "Vitals" => Ok(PatientsMethods::Vitals(VitalsMethods::from_name(
                        &elements.join("::"),
                        data,
                    )?)),
                    "Allergies" => Ok(PatientsMethods::Allergy(AllergyMethods::from_name(
                        &elements.join("::"),
                        data,
                    )?)),
                    "Problems" => Ok(PatientsMethods::Problems(ProblemsMethods::from_name(
                        &elements.join("::"),
                        data,
                    )?)),
                    "Order" => Ok(PatientsMethods::Order(OrderMethods::from_name(
                        &elements.join("::"),
                        data,
                    )?)),
                    "Notes" => Ok(PatientsMethods::Notes(NotesMethods::from_name(
                        &elements.join("::"),
                        data,
                    )?)),
                    "AddHistorical" => Ok(PatientsMethods::AddHistorical(
                        AddhisitoricalMethods::from_name(&elements.join("::"), data)?,
                    )),
                    "Addminister" => Ok(PatientsMethods::Addminister(
                        AddministerMethods::from_name(&elements.join("::"), data)?,
                    )),
                    "Notaddminister" => Ok(PatientsMethods::Notaddminister(
                        NotaddministerMethods::from_name(&elements.join("::"), data)?,
                    )),
                    "Medications" => Ok(PatientsMethods::Medications(
                        MedicationsMethods::from_name(&elements.join("::"), data)?,
                    )),
                    "History" => Ok(PatientsMethods::History(HistoryMethods::from_name(
                        &elements.join("::"),
                        data,
                    )?)),
                    _ => Err(ErrorData::std(-32601)),
                }
            } else {
                Err(ErrorData::std(-32601))
            };

        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        match self {
            PatientsMethods::Add(input) => create_request(namespace, "Add", input),
            PatientsMethods::Update(input) => create_request(namespace, "Update", input),
            PatientsMethods::Delete(input) => create_request(namespace, "Delete", input),
            PatientsMethods::Vitals(vitals) => {
                vitals.to_rpc(&format!("{}::{}", namespace, "Vitals"))
            }
            PatientsMethods::Allergy(allergy) => {
                allergy.to_rpc(&format!("{}::{}", namespace, "Allergies"))
            }
            PatientsMethods::Problems(problem) => {
                problem.to_rpc(&format!("{}::{}", namespace, "Problems"))
            }
            PatientsMethods::Order(order) => order.to_rpc(&format!("{}::{}", namespace, "Order")),
            PatientsMethods::Notes(notes) => notes.to_rpc(&format!("{}::{}", namespace, "Notes")),
            PatientsMethods::AddHistorical(addhistorical) => {
                addhistorical.to_rpc(&format!("{}::{}", namespace, "AddHistorical"))
            }
            PatientsMethods::Addminister(addminister) => {
                addminister.to_rpc(&format!("{}::{}", namespace, "Addminister"))
            }
            PatientsMethods::Notaddminister(notadminister) => {
                notadminister.to_rpc(&format!("{}::{}", namespace, "Notaddminister"))
            }
            PatientsMethods::Medications(medications) => {
                medications.to_rpc(&format!("{}::{}", namespace, "Medications"))
            }
            PatientsMethods::History(history) => {
                history.to_rpc(&format!("{}::{}", namespace, "History"))
            }
        }
    }
}
