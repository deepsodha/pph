use anyhow::{Ok, Result};
use log::info;

use sqlx::{Pool, Sqlite};

use crate::setup::prelude::{
    add_historical_table_read::create_addhistorical_state_table,
    administer_table_read::create_administer_state_table,
    allergies_table_read::create_allergies_state_table,
    appointment_table_read::create_appointment_state_table, auth_table_read::create_auth_table,
    doctor_table_read::create_doctor_state_table,
    family_history_table_read::create_familyhistory_state_table,
    hospitalization_table_read::create_hospitalization_state_table,
    implantabledevices_table_read::create_implantabledevices_state_table,
    medication_table_read::create_medication_state_table,
    not_administer_table_read::create_notadminister_state_table,
    notes_table_read::create_notes_state_table,
    obandpregnancy_table_read::create_obandpregnancy_state_table,
    order_table_read::create_order_state_table,
    organization_table_read::create_organization_state_table,
    pastmedical_history_tabl_read::create_pastmedicalhistory_state_table,
    pastsurgical_history_table_read::create_pastsurgicalhistory_state_table,
    patient_problem_table_read::create_problem_state_table,
    patient_table_read::create_patient_state_table,
    prescription_table_read::create_prescription_state_table, roles_table_read::create_roles_table,
    servicelocation_table_read::create_servicelocation_state_table,
    social_history_table_read::create_socialhistory_state_table,
    staff_table_read::create_staff_state_table,
    systemadmin_table_read::create_systemadmin_state_table,
    user_table_read::create_user_state_table, vitals_table_read::create_vitals_state_table,
};

pub async fn dev_setup(read_pool: Pool<Sqlite>) -> Result<()> {
    info!(target: "Setup", "Setting up dev Env");
    create_auth_table(read_pool.clone()).await?;
    create_roles_table(read_pool.clone()).await?;
    create_patient_state_table(read_pool.clone()).await?;
    create_addhistorical_state_table(read_pool.clone()).await?;
    create_administer_state_table(read_pool.clone()).await?;
    create_allergies_state_table(read_pool.clone()).await?;
    create_appointment_state_table(read_pool.clone()).await?;
    create_doctor_state_table(read_pool.clone()).await?;
    create_medication_state_table(read_pool.clone()).await?;
    create_notadminister_state_table(read_pool.clone()).await?;
    create_order_state_table(read_pool.clone()).await?;
    create_organization_state_table(read_pool.clone()).await?;
    create_problem_state_table(read_pool.clone()).await?;
    create_prescription_state_table(read_pool.clone()).await?;
    create_servicelocation_state_table(read_pool.clone()).await?;
    create_systemadmin_state_table(read_pool.clone()).await?;
    create_vitals_state_table(read_pool.clone()).await?;
    create_familyhistory_state_table(read_pool.clone()).await?;
    create_hospitalization_state_table(read_pool.clone()).await?;
    create_implantabledevices_state_table(read_pool.clone()).await?;
    create_obandpregnancy_state_table(read_pool.clone()).await?;
    create_pastmedicalhistory_state_table(read_pool.clone()).await?;
    create_pastsurgicalhistory_state_table(read_pool.clone()).await?;
    create_socialhistory_state_table(read_pool.clone()).await?;
    create_staff_state_table(read_pool.clone()).await?;
    create_user_state_table(read_pool.clone()).await?;
    create_notes_state_table(read_pool.clone()).await?;
    // create_icd10_table(read_pool.clone()).await?;
    // create_combined_table(read_pool.clone()).await?;
    //TODO: By Pass FU Check and Create First User if Not There
    //TODO: Set up Events it is triggered from ENV file ( some variable to reset data for dev only)
    //TODO: Set up Read Database if Events are new depends on ENV file ( as above )
    Ok(())
}
