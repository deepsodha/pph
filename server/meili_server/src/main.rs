use common::dto::{doctor_details::DoctorDetails, drugs::Drugs, icd::ICD10, location::Location};
use dotenv::var;
use futures::executor::block_on;
use helper::{read_doctor_data, read_drugs_data, read_icd_data, read_location_data};
use meilisearch_sdk::client::*;
pub mod helper;

fn main() {
    let api_key = var("MEILI_MASTER_KEY").expect("MEILI_MASTER_KEY must be set");
    block_on(async {
        let client = Client::new("http://localhost:7700", Some(api_key));

        // Process drugs data
        let drugs_docs: Vec<Drugs> = read_drugs_data().await;
        client
            .index("drugs")
            .add_documents(&drugs_docs, None)
            .await
            .unwrap();

        // Process doctor data
        let doctor_details: Vec<DoctorDetails> = read_doctor_data().await;
        client
            .index("doctors")
            .add_documents(&doctor_details, None)
            .await
            .unwrap();

        // Process icd data
        let icd_details: Vec<ICD10> = read_icd_data().await;
        client
            .index("icd")
            .add_documents(&icd_details, None)
            .await
            .unwrap();

        // Process Location data
        let location_details: Vec<Location> = read_location_data().await;
        client
            .index("location")
            .add_documents(&location_details, None)
            .await
            .unwrap();
    });
}
