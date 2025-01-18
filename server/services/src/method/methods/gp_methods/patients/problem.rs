use crate::method::convention::{ErrorData, Request};
use crate::rpc::create_request;
use crate::rpc::Rpc;
use patient::dto::problem::problem_add::ProblemsAdd;
use patient::dto::problem::problem_delete::ProblemsDelete;
use patient::dto::problem::problem_update::ProblemsUpdate;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum ProblemsMethods {
    Add(ProblemsAdd, Option<String>),
    Update(ProblemsUpdate, Option<String>),
    Delete(ProblemsDelete, Option<String>),
}

impl Rpc for ProblemsMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        //Add as str
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<ProblemsAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(ProblemsMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<ProblemsUpdate>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(ProblemsMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<ProblemsDelete>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(ProblemsMethods::Delete(input, None))
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
            ProblemsMethods::Add(input, _) => create_request(namespace, "Add", input),
            ProblemsMethods::Update(input, _) => create_request(namespace, "Update", input),

            ProblemsMethods::Delete(input, _) => create_request(namespace, "Delete", input),
        }
    }
}
