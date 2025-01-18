use crate::method::convention::{ErrorData, Request};
use crate::rpc::{create_request, Rpc};
use patient::dto::history::implantabledevices::implantabledevices_add::ImplantableDevicesAdd;
use patient::dto::history::implantabledevices::implantabledevices_delete::ImplantableDevicesDelete;
use patient::dto::history::implantabledevices::implantabledevices_update::ImplantableDevicesUpdate;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum ImplantableDevicesMethods {
    Add(ImplantableDevicesAdd),
    Update(ImplantableDevicesUpdate),
    Delete(ImplantableDevicesDelete),
}

impl Rpc for ImplantableDevicesMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        //Add as str
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<ImplantableDevicesAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(ImplantableDevicesMethods::Add(input))
                }
                "Update" => {
                    let input = serde_json::from_value::<ImplantableDevicesUpdate>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(ImplantableDevicesMethods::Update(input))
                }
                "Delete" => {
                    let input = serde_json::from_value::<ImplantableDevicesDelete>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(ImplantableDevicesMethods::Delete(input))
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
            ImplantableDevicesMethods::Add(input) => create_request(namespace, "Add", input),

            ImplantableDevicesMethods::Update(input) => create_request(namespace, "Update", input),

            ImplantableDevicesMethods::Delete(input) => create_request(namespace, "Delete", input),
        }
    }
}
