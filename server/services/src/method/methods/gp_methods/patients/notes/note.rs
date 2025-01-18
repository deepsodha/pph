use crate::method::convention::{ErrorData, Request};
use crate::rpc::Rpc;
use serde_json::Value;

use super::acupuncture_follow_up::AcupunctureFollowUpMethods;
use super::amendment::AmendmentMethods;
use super::group::GroupMethods;
use super::history_and_physical::HistoryAndPhysicalMethods;
use super::med_spa_procedure::MedSpaProcedureMethods;
use super::office_form::OfficeFormMethods;
use super::phone::PhoneMethods;
use super::soap::SoapMethods;

#[derive(PartialEq, Eq, Debug)]
pub enum NotesMethods {
    Soap(Box<SoapMethods>),
    HistoryAndPhysical(Box<HistoryAndPhysicalMethods>),
    AcupunctureFollowUp(Box<AcupunctureFollowUpMethods>),
    Amendment(Box<AmendmentMethods>),
    Group(Box<GroupMethods>),
    OfficeForm(Box<OfficeFormMethods>),
    Phone(Box<PhoneMethods>),
    MedSpaProcedure(Box<MedSpaProcedureMethods>),
}

impl Rpc for NotesMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        //Add as str
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, elements)) = names.split_first() {
            match *first {
                "Soap" => Ok(NotesMethods::Soap(Box::new(SoapMethods::from_name(
                    &elements.join("::"),
                    data,
                )?))),
                "HistoryAndPhysical" => Ok(NotesMethods::HistoryAndPhysical(Box::new(
                    HistoryAndPhysicalMethods::from_name(&elements.join("::"), data)?,
                ))),
                "AcupunctureFollowUp" => Ok(NotesMethods::AcupunctureFollowUp(Box::new(
                    AcupunctureFollowUpMethods::from_name(&elements.join("::"), data)?,
                ))),
                "Amendment" => Ok(NotesMethods::Amendment(Box::new(
                    AmendmentMethods::from_name(&elements.join("::"), data)?,
                ))),
                "Group" => Ok(NotesMethods::Group(Box::new(GroupMethods::from_name(
                    &elements.join("::"),
                    data,
                )?))),
                "MedSpaProcedure" => Ok(NotesMethods::MedSpaProcedure(Box::new(
                    MedSpaProcedureMethods::from_name(&elements.join("::"), data)?,
                ))),
                "OfficeForm" => Ok(NotesMethods::OfficeForm(Box::new(
                    OfficeFormMethods::from_name(&elements.join("::"), data)?,
                ))),
                "Phone" => Ok(NotesMethods::Phone(Box::new(PhoneMethods::from_name(
                    &elements.join("::"),
                    data,
                )?))),
                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };

        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        match self {
            NotesMethods::Soap(soap) => soap.to_rpc(&format!("{}::{}", namespace, "Soap")),

            NotesMethods::HistoryAndPhysical(historyandphysical) => {
                historyandphysical.to_rpc(&format!("{}::{}", namespace, "HistoryAndPhysical"))
            }
            NotesMethods::AcupunctureFollowUp(acupuncture_follow_up) => {
                acupuncture_follow_up.to_rpc(&format!("{}::{}", namespace, "AcupuntureFollowUp"))
            }
            NotesMethods::Amendment(amendment) => {
                amendment.to_rpc(&format!("{}::{}", namespace, "Amendment"))
            }
            NotesMethods::Group(group) => group.to_rpc(&format!("{}::{}", namespace, "Group")),
            NotesMethods::MedSpaProcedure(medspaprocedure) => {
                medspaprocedure.to_rpc(&format!("{}::{}", namespace, "MedSpaProcedure"))
            }
            NotesMethods::OfficeForm(officeform) => {
                officeform.to_rpc(&format!("{}::{}", namespace, "OfficeForm"))
            }
            NotesMethods::Phone(phone) => phone.to_rpc(&format!("{}::{}", namespace, "Phone")),
        }
    }
}
