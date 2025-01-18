use crate::doc::syncs::DataTable;
use anyhow::Result;
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use csv::ReaderBuilder;
use db_services::services::organization::upsert_organization;
use organization::domain::{
    organization_aggregate::ORGANIZATION_AGGREGATE, organization_events::OrganizationEvent,
};
use organization::dto::csv::{CombinedRecord, Icd10Record, Location, PhoneCode, VaccineRecord};
use serde_json::Value;
use sqlx::{Pool, Sqlite};
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::Read;

pub fn read_json_from_file(file_path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut json_string = String::new();
    file.read_to_string(&mut json_string)?;
    let json_data: Value = serde_json::from_str(&json_string)?;
    Ok(json_data)
}

pub fn get_all_vaccines(records: &[VaccineRecord]) -> Vec<(&str, &str)> {
    let unique_vaccines: HashSet<(&str, &str)> = records
        .iter()
        .map(|record| (record.vaccine.as_str(), record.vaccine_type.as_str()))
        .collect();

    let vaccines = unique_vaccines.into_iter().collect::<Vec<(&str, &str)>>();

    log::info!("Retrieved all vaccines");

    vaccines
}

pub fn read_csv(file_path: &str) -> Result<Vec<Location>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    let records = rdr
        .records()
        .filter_map(|record| {
            record.map_or_else(
                |e| {
                    log::error!("Error reading CSV record: {}", e);
                    None
                },
                Some,
            )
        })
        .map(|row| Location {
            city_name: row[2].to_string(),
            state_name: row[0].to_string(),
            country_name: row[4].to_string(),
            pin_code: row[3].to_string(),
            district: row[1].to_string(),
        })
        .collect::<Vec<_>>();

    log::info!("CSV file read successfully");

    Ok(records)
}

pub fn read_vaccine_csv(file_path: &str) -> Result<Vec<VaccineRecord>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    let records = rdr
        .records()
        .filter_map(|record| {
            record.map_or_else(
                |e| {
                    log::error!("Error reading CSV record: {}", e);
                    None
                },
                Some,
            )
        })
        .map(|row| VaccineRecord {
            vaccine: row[0].to_string(),
            vaccine_type: row[1].to_string(),
        })
        .collect::<Vec<_>>();

    log::info!("CSV file read successfully");

    Ok(records)
}

pub fn read_combined_csv(file_path: &str) -> Result<Vec<CombinedRecord>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    let records = rdr
        .records()
        .filter_map(|record| {
            record.map_or_else(
                |e| {
                    log::error!("Error reading CSV record: {}", e);
                    None
                },
                Some,
            )
        })
        .map(|row| CombinedRecord {
            brand: row[0].to_string(),
            generic: row[1].to_string(),
            details: row[2].to_string(),
            category: row[3].to_string(),
            side_effects: row[4].to_string(),
            drug_dose_info: row[5].to_string(),
            precautions: row[6].to_string(),
            manufacturer_name: row[7].to_string(),
            medicines: row[8].to_string(),
            contra_indications: row[9].to_string(),
            diseases: row[10].to_string(),
            interactions: row[11].to_string(),
            contains: row[12].to_string(),
        })
        .collect::<Vec<_>>();

    log::info!("CSV file read successfully");

    Ok(records)
}

pub fn read_icd10_csv(file_path: &str) -> Result<Vec<Icd10Record>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    let records = rdr
        .records()
        .filter_map(|record| {
            record.map_or_else(
                |e| {
                    log::error!("Error reading CSV record: {}", e);
                    None
                },
                Some,
            )
        })
        .map(|row| Icd10Record {
            disease_code: row[0].to_string(),
            disease_subtype: row[1].to_string(),
            disease_subcode: row[2].to_string(),
            disease_name: row[3].to_string(),
            disease_description: row[4].to_string(),
            disease_category: row[5].to_string(),
        })
        .collect::<Vec<_>>();

    log::info!("CSV file read successfully");

    Ok(records)
}

pub async fn process_organization_events(
    read_pool: Pool<Sqlite>,
    organization_id: String,
    stream_id: String,
    read_events: Vec<EventRead<OrganizationEvent, OrganizationEvent, EventVersion>>,
) -> Result<()> {
    log::info!(
        "Processing organization events for organization_id: {}",
        organization_id
    );

    let organization_db = sqlx::query_as::<_, DataTable>(
        "SELECT * from organization_table_state WHERE id = ? LIMIT 1",
    )
    .bind(organization_id.clone())
    .fetch_optional(&read_pool)
    .await?;

    let organization_state =
        organization_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());

    let organization_updated_state = read_events.iter().fold(organization_state, |a, b| {
        ORGANIZATION_AGGREGATE.apply(a, &b.data)
    });

    match organization_updated_state {
        Some(p) => {
            upsert_organization(
                read_pool,
                p,
                read_events
                    .last()
                    .map_or_else(|| 0, |event| event.version.0),
                stream_id,
            )
            .await?;
        }
        None => {
            log::error!("Organization with id {} not found", organization_id);
        }
    }

    log::info!(
        "Processing organization events completed for organization_id: {}",
        organization_id
    );

    Ok(())
}

pub fn read_phonecode_csv(file_path: &str) -> Result<Vec<PhoneCode>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    let records = rdr
        .records()
        .filter_map(|record| {
            record.map_or_else(
                |e| {
                    log::error!("Error reading CSV record: {}", e);
                    None
                },
                Some,
            )
        })
        .map(|row| PhoneCode {
            phone_code: row[2].to_string(),
        })
        .collect::<Vec<_>>();

    log::info!("CSV file read successfully");

    Ok(records)
}
