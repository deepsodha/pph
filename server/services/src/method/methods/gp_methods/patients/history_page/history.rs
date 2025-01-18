use super::familyhistory::FamilyHistoryMethods;
use super::hospitalization::HospitalizationMethods;
use super::implantabledevices::ImplantableDevicesMethods;
use super::obandpregnancy::OBAndPregnancyMethods;
use super::pastmedicalhistory::PastMedicalHistoryMethods;
use super::pastsurgicalhistory::PastSurgicalHistoryMethods;
use super::socialhistory::SocialHistoryMethods;
use crate::method::convention::{ErrorData, Request};
use crate::rpc::Rpc;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum HistoryMethods {
    FamilyHistory(FamilyHistoryMethods),
    Hospitalization(HospitalizationMethods),
    ImplantableDevices(ImplantableDevicesMethods),
    PastMedicalHistory(Box<PastMedicalHistoryMethods>),
    ObAndPregnancy(OBAndPregnancyMethods),
    PastSurgicalHistory(PastSurgicalHistoryMethods),
    SocialHistory(SocialHistoryMethods),
}

impl Rpc for HistoryMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        //Add as str
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, elements)) = names.split_first() {
            match *first {
                "FamilyHistory" => Ok(HistoryMethods::FamilyHistory(
                    FamilyHistoryMethods::from_name(&elements.join("::"), data)?,
                )),
                "Hospitalization" => Ok(HistoryMethods::Hospitalization(
                    HospitalizationMethods::from_name(&elements.join("::"), data)?,
                )),
                "ImplantableDevices" => Ok(HistoryMethods::ImplantableDevices(
                    ImplantableDevicesMethods::from_name(&elements.join("::"), data)?,
                )),
                "PastMedicalHistory" => {
                    let boxed_method = Box::new(PastMedicalHistoryMethods::from_name(
                        &elements.join("::"),
                        data,
                    )?);
                    Ok(HistoryMethods::PastMedicalHistory(boxed_method))
                }
                "ObAndPregnancy" => Ok(HistoryMethods::ObAndPregnancy(
                    OBAndPregnancyMethods::from_name(&elements.join("::"), data)?,
                )),
                "PastSurgicalHistory" => Ok(HistoryMethods::PastSurgicalHistory(
                    PastSurgicalHistoryMethods::from_name(&elements.join("::"), data)?,
                )),
                "SocialHistory" => Ok(HistoryMethods::SocialHistory(
                    SocialHistoryMethods::from_name(&elements.join("::"), data)?,
                )),

                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };

        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        match self {
            HistoryMethods::FamilyHistory(familyhistory) => {
                familyhistory.to_rpc(&format!("{}::{}", namespace, "FamilyHistory"))
            }

            HistoryMethods::Hospitalization(hospitalization) => {
                hospitalization.to_rpc(&format!("{}::{}", namespace, "Hospitalization"))
            }
            HistoryMethods::ImplantableDevices(implantabledevices) => {
                implantabledevices.to_rpc(&format!("{}::{}", namespace, "ImplantableDevices"))
            }

            HistoryMethods::PastMedicalHistory(pastmedicalhistory) => {
                pastmedicalhistory.to_rpc(&format!("{}::{}", namespace, "PastMedicalHistory"))
            }

            HistoryMethods::ObAndPregnancy(obandpregnancy) => {
                obandpregnancy.to_rpc(&format!("{}::{}", namespace, "ObAndPregnancy"))
            }

            HistoryMethods::PastSurgicalHistory(pastsurgicalhistory) => {
                pastsurgicalhistory.to_rpc(&format!("{}::{}", namespace, "PastSurgicalHistory"))
            }

            HistoryMethods::SocialHistory(socialhistory) => {
                socialhistory.to_rpc(&format!("{}::{}", namespace, "SocialHistory"))
            }
        }
    }
}
