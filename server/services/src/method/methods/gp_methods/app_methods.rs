use super::appointments::appointment::AppointmentMethods;
use super::doctors::doctor::DoctorMethods;
use super::organizations::organization::OrganizationMethods;
use super::patients::patient::PatientsMethods;
use super::prescriptions::prescription::PrescriptionMethods;
use super::servicelocation::service_location::ServiceLocationMethods;
use super::staffs::staff::StaffMethods;
use super::system_admin::SystemAdminMethods;
use crate::method::convention::{ErrorData, Request};
use crate::method::methods::gp_methods::user::users::UserMethods;
use crate::rpc::Rpc;
use log::info;

pub enum AppMethods {
    Patients(PatientsMethods),
    Doctors(DoctorMethods),
    Appointments(AppointmentMethods),
    Prescription(PrescriptionMethods),
    Organizations(OrganizationMethods),
    SystemAdmin(SystemAdminMethods),
    ServiceLocation(ServiceLocationMethods),
    Staff(StaffMethods),
    User(UserMethods),
}

impl Rpc for AppMethods {
    fn from_name(str: &str, data: Vec<serde_json::Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        info!(target: "AppMethods::from_name", "str: {:#?}, data: {:#?}", str, data);
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, elements)) = names.split_first() {
            match *first {
                "Patients" => Ok(AppMethods::Patients(PatientsMethods::from_name(
                    &elements.join("::"),
                    data,
                )?)),
                "Doctor" => Ok(AppMethods::Doctors(DoctorMethods::from_name(
                    &elements.join("::"),
                    data,
                )?)),
                "Appointment" => Ok(AppMethods::Appointments(AppointmentMethods::from_name(
                    &elements.join("::"),
                    data,
                )?)),
                "Prescription" => Ok(AppMethods::Prescription(PrescriptionMethods::from_name(
                    &elements.join("::"),
                    data,
                )?)),
                "Organization" => Ok(AppMethods::Organizations(OrganizationMethods::from_name(
                    &elements.join("::"),
                    data,
                )?)),
                "SystemAdmin" => Ok(AppMethods::SystemAdmin(SystemAdminMethods::from_name(
                    &elements.join("::"),
                    data,
                )?)),
                "ServiceLocation" => Ok(AppMethods::ServiceLocation(
                    ServiceLocationMethods::from_name(&elements.join("::"), data)?,
                )),
                "Staff" => Ok(AppMethods::Staff(StaffMethods::from_name(
                    &elements.join("::"),
                    data,
                )?)),
                "User" => Ok(AppMethods::User(UserMethods::from_name(
                    &elements.join("::"),
                    data,
                )?)),

                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };
        res
    }
    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        info!("{}", namespace);
        match self {
            AppMethods::Patients(patients) => patients.to_rpc("Patients"),
            AppMethods::Doctors(doctor) => doctor.to_rpc("Doctor"),
            AppMethods::Appointments(appointment) => appointment.to_rpc("Appointment"),
            AppMethods::Prescription(prescription) => prescription.to_rpc("Prescription"),
            AppMethods::Organizations(organization) => organization.to_rpc("Organization"),
            AppMethods::SystemAdmin(systemadmin) => systemadmin.to_rpc("SystemAdmin"),
            AppMethods::ServiceLocation(servicelocation) => {
                servicelocation.to_rpc("ServiceLocation")
            }
            AppMethods::Staff(staff) => staff.to_rpc("Staff"),
            AppMethods::User(user) => user.to_rpc("User"),
        }
    }
}
