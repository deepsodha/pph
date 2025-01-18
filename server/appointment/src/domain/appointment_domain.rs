use super::{
    appointment_commands::{CreateAppointment, DeleteAppointment, UpdateAppointment},
    appointment_events::{AppointmentCreated, AppointmentUpdated},
};
use crate::dto::{
    appointment_choose_type::ChooseAppointmentType, appointment_visit_type::VisitType,
};
use anyhow::Result;
use chrono::{DateTime, Utc};
use common::domain::{
    appointment_duration_range::AppointmentDuration, name::Name, required_string::RequiredString,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

impl From<AppointmentCreated> for AppointmentState {
    fn from(u: AppointmentCreated) -> Self {
        AppointmentState {
            id: String::from(&u.id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            service_location_id: String::from(&u.service_location_id),
            doctor_id: String::from(&u.doctor_id),
            doctor_name: u.doctor_name,
            patient_name: String::from(&u.patient_name),
            visit: u.visit,
            date: u.date,
            appointment_duration: u.appointment_duration,
            choose_appointment: u.choose_appointment,
            note: u.note,
            room_and_equipment_no: u.room_and_equipment_no,
            staff_id: u.staff_id,
            staff_name: u.staff_name,
            is_deleted: false,
        }
    }
}

impl From<AppointmentUpdated> for AppointmentState {
    fn from(u: AppointmentUpdated) -> Self {
        AppointmentState {
            id: String::from(&u.id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            doctor_id: String::from(&u.doctor_id),
            service_location_id: String::from(&u.service_location_id),
            doctor_name: u.doctor_name,
            patient_name: String::from(&u.patient_name),
            visit: u.visit,
            date: u.date,
            appointment_duration: u.appointment_duration,
            choose_appointment: u.choose_appointment,
            note: u.note,
            room_and_equipment_no: u.room_and_equipment_no,
            staff_id: u.staff_id,
            staff_name: u.staff_name,
            is_deleted: false,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export, export_to = "../../bindings/AppointmentState.ts")]
pub struct AppointmentState {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_id: String,
    pub doctor_id: String,
    pub service_location_id: String,
    pub doctor_name: Option<String>,
    pub patient_name: String,
    pub visit: VisitType,
    pub date: DateTime<Utc>,
    pub appointment_duration: u32,
    pub choose_appointment: ChooseAppointmentType,
    pub note: String,
    pub room_and_equipment_no: String,
    pub staff_id: String,
    pub staff_name: String,
    pub is_deleted: bool,
}

#[derive(Debug, Clone)]
pub struct Create {
    pub id: String,
    pub service_location_id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: RequiredString,
    pub patient_id: RequiredString,
    pub doctor_id: RequiredString,
    pub doctor: Option<String>,
    pub patient_name: Name,
    pub visit: VisitType,
    pub date: DateTime<Utc>,
    pub appointment_duration: AppointmentDuration,
    pub choose_appointment: ChooseAppointmentType,
    pub note: RequiredString,
    pub room_and_equipment_no: RequiredString,
    pub staff_id: RequiredString,
    pub staff_name: RequiredString,
}

#[derive(Debug, Clone)]
pub struct Update {
    pub id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: RequiredString,
    pub patient_id: RequiredString,
    pub doctor_id: RequiredString,
    pub service_location_id: RequiredString,
    pub doctor: Option<String>,
    pub patient_name: Name,
    pub visit: VisitType,
    pub date: DateTime<Utc>,
    pub appointment_duration: AppointmentDuration,
    pub choose_appointment: ChooseAppointmentType,
    pub note: RequiredString,
    pub room_and_equipment_no: RequiredString,
    pub staff_id: RequiredString,
    pub staff_name: RequiredString,
}

#[derive(Debug, Clone)]
pub struct Delete {
    pub id: RequiredString,
    pub patient_id: RequiredString,
    pub doctor_id: RequiredString,
    pub org_id: RequiredString,
    pub service_location_id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl Create {
    pub fn parse(a: &CreateAppointment) -> Result<Create> {
        Ok(Create {
            id: a.id.to_owned(),
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            service_location_id: RequiredString::parse(&a.service_location_id)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            org_id: RequiredString::parse(&a.org_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
            doctor_id: RequiredString::parse(&a.doctor_id)?,
            doctor: (a.doctor_name).to_owned(),
            patient_name: Name::parse(&a.patient_name, "Patient")?,
            visit: (a.visit).to_owned(),
            date: (a.date).to_owned(),
            appointment_duration: AppointmentDuration::parse(&a.appointment_duration)?,
            choose_appointment: (a.choose_appointment).to_owned(),
            note: RequiredString::parse(&a.note)?,
            room_and_equipment_no: RequiredString::parse(&a.room_and_equipment_no)?,
            staff_id: RequiredString::parse(&a.staff_id)?,
            staff_name: RequiredString::parse(&a.staff_name)?,
        })
    }
}

impl Update {
    pub fn parse(a: &UpdateAppointment) -> Result<Update> {
        Ok(Update {
            id: RequiredString::parse(&a.id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            service_location_id: RequiredString::parse(&a.service_location_id)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            org_id: RequiredString::parse(&a.org_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
            doctor_id: RequiredString::parse(&a.doctor_id)?,
            doctor: (a.doctor_name).to_owned(),
            patient_name: Name::parse(&a.patient_name, "Patient")?,
            visit: (a.visit).to_owned(),
            date: (a.date).to_owned(),
            appointment_duration: AppointmentDuration::parse(&a.appointment_duration)?,
            choose_appointment: (a.choose_appointment).to_owned(),
            note: RequiredString::parse(&a.note)?,
            room_and_equipment_no: RequiredString::parse(&a.room_and_equipment_no)?,
            staff_id: RequiredString::parse(&a.staff_id)?,
            staff_name: RequiredString::parse(&a.staff_name)?,
        })
    }
}
impl Delete {
    pub fn parse(a: &DeleteAppointment) -> Result<Delete> {
        Ok(Delete {
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            id: RequiredString::parse(&a.id)?,
            doctor_id: RequiredString::parse(&a.doctor_id)?,
            service_location_id: RequiredString::parse(&a.service_location_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
            org_id: RequiredString::parse(&a.org_id)?,
        })
    }
}
