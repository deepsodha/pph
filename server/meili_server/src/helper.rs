use common::dto::location::Location;
use common::dto::{doctor_details::DoctorDetails, drugs::Drugs, icd::ICD10};
use std::fs::File;
use std::io::Read;

pub async fn read_icd_data() -> Vec<ICD10> {
    let mut file = File::open(".././statics/data/icd10.json").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    match serde_json::from_str(&content) {
        Ok(data) => data,
        Err(err) => {
            if let serde_json::error::Category::Data = err.classify() {
                panic!("The JSON data format is incorrect: {:?}", err);
            } else {
                panic!("An error occurred while deserializing JSON: {:?}", err);
            }
        }
    }
}

pub async fn read_drugs_data() -> Vec<Drugs> {
    let mut file = File::open(".././statics/data/drugs.json").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    match serde_json::from_str(&content) {
        Ok(data) => data,
        Err(err) => {
            if let serde_json::error::Category::Data = err.classify() {
                panic!("The JSON data format is incorrect: {:?}", err);
            } else {
                panic!("An error occurred while deserializing JSON: {:?}", err);
            }
        }
    }
}

pub async fn read_doctor_data() -> Vec<DoctorDetails> {
    let mut file = File::open(".././statics/data/doctor.json").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let doctor_data: serde_json::Value = match serde_json::from_str(&content) {
        Ok(data) => data,
        Err(err) => {
            if let serde_json::error::Category::Data = err.classify() {
                panic!("The JSON data format is incorrect: {:?}", err);
            } else {
                panic!("An error occurred while deserializing JSON: {:?}", err);
            }
        }
    };

    serde_json::from_value(doctor_data["posts"].clone()).unwrap()
}

pub async fn read_location_data() -> Vec<Location> {
    let mut file = File::open(".././statics/data/locations.json").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    match serde_json::from_str(&content) {
        Ok(data) => data,

        Err(err) => {
            if let serde_json::error::Category::Data = err.classify() {
                panic!("The JSON data format is incorrect: {:?}", err);
            } else {
                panic!("An error occurred while deserializing JSON: {:?}", err);
            }
        }
    }
}
