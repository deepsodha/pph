use chrono::{DateTime, Utc};
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dto::{
    appointment_choose_type::ChooseAppointmentType, appointment_visit_type::VisitType,
};

use super::appointment_domain::{Create, Delete, Update};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AppointmentEvent {
    AppointmentCreated(AppointmentCreated),
    AppointmentUpdated(AppointmentUpdated),
    AppointmentDeleted(AppointmentDeleted),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppointmentCreated {
    pub id: String,
    pub org_id: String,
    pub service_location_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub patient_id: String,
    pub doctor_id: String,
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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppointmentUpdated {
    pub id: String,
    pub org_id: String,
    pub service_location_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub patient_id: String,
    pub doctor_id: String,
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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppointmentDeleted {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub doctor_id: String,
    pub service_location_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl From<AppointmentEvent> for EventWrite<AppointmentEvent, AppointmentEvent> {
    fn from(u: AppointmentEvent) -> Self {
        EventWrite {
            id: Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("appointment_event"),
            data: u,
            metadata: None,
        }
    }
}

impl From<Create> for AppointmentCreated {
    fn from(s: Create) -> Self {
        AppointmentCreated {
            id: s.id,
            org_id: String::from(s.org_id.as_ref()),
            service_location_id: String::from(s.service_location_id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            patient_id: String::from(s.patient_id.as_ref()),
            doctor_id: String::from(s.doctor_id.as_ref()),
            doctor_name: s.doctor,
            patient_name: String::from(s.patient_name.as_ref()),
            visit: s.visit,
            date: s.date,
            appointment_duration: *s.appointment_duration.as_ref(),
            choose_appointment: s.choose_appointment,
            note: String::from(s.note.as_ref()),
            room_and_equipment_no: String::from(s.room_and_equipment_no.as_ref()),
            staff_id: String::from(s.staff_id.as_ref()),
            staff_name: String::from(s.staff_name.as_ref()),
        }
    }
}

impl From<Update> for AppointmentUpdated {
    fn from(s: Update) -> Self {
        AppointmentUpdated {
            id: String::from(s.id.as_ref()),
            org_id: String::from(s.org_id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            service_location_id: String::from(s.service_location_id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
            doctor_id: String::from(s.doctor_id.as_ref()),
            doctor_name: s.doctor,
            patient_name: String::from(s.patient_name.as_ref()),
            visit: s.visit,
            date: s.date,
            appointment_duration: *s.appointment_duration.as_ref(),
            choose_appointment: s.choose_appointment,
            note: String::from(s.note.as_ref()),
            room_and_equipment_no: String::from(s.room_and_equipment_no.as_ref()),
            staff_id: String::from(s.staff_id.as_ref()),
            staff_name: String::from(s.staff_name.as_ref()),
        }
    }
}

impl From<Delete> for AppointmentDeleted {
    fn from(s: Delete) -> Self {
        AppointmentDeleted {
            id: String::from(s.id.as_ref()),
            doctor_id: String::from(s.doctor_id.as_ref()),
            service_location_id: String::from(s.service_location_id.as_ref()),
            org_id: String::from(s.org_id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
        }
    }
}
