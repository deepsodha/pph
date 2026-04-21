use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Default, Clone, Validate, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/LastUpdatedInput.ts")]
pub struct LastUpdatedInput {
    pub system_admin: DateTime<Utc>,
    pub doctors: DateTime<Utc>,
    pub patients: DateTime<Utc>,
    pub appointments: DateTime<Utc>,
    pub prescription: DateTime<Utc>,
    pub service_location: DateTime<Utc>,
    pub add_historical: DateTime<Utc>,
    pub administer: DateTime<Utc>,
    pub allergy: DateTime<Utc>,
    pub medication: DateTime<Utc>,
    pub not_administer: DateTime<Utc>,
    pub order: DateTime<Utc>,
    pub problems: DateTime<Utc>,
    pub vitals: DateTime<Utc>,
    pub familyhistory: DateTime<Utc>,
    pub hospitalization: DateTime<Utc>,
    pub implantabledevices: DateTime<Utc>,
    pub obandpregnanacy: DateTime<Utc>,
    pub pastmedicalhistory: DateTime<Utc>,
    pub pastsurgicalhistory: DateTime<Utc>,
    pub socialhistory: DateTime<Utc>,
    pub staff: DateTime<Utc>,
    pub note: DateTime<Utc>,
    pub user: DateTime<Utc>,
    pub organization: DateTime<Utc>,
    /// Maximum records to return per table. Defaults to 500 when absent.
    /// Use the latest `last_updated` timestamp from the returned records as the
    /// cursor for the next page.
    #[serde(default)]
    pub limit: Option<i64>,
}
