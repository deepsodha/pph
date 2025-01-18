use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::domain::{
    allergies::allergies_domain::AllergiesState, medication::medication_domain::MedicationsState,
    vital::vital_domain::VitalsState,
};

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/NoteType.ts")]
pub enum NoteType {
    HistoryAndPhysical(Box<HistoryAndPhysical>),
    SOAPNote(SOAPNote),
    AcupunctureFollowUp(AcupunctureFollowUp),
    AcupunctureInitialVisit(AcupunctureInitialVisit),
    AcupunctureReassessment(AcupunctureReassessment),
    Amendment(Amendment),
    Consultation(Consultation),
    DischargeSummary(DischargeSummary),
    Group(Group),
    MedSpaIVProcedure(MedSpaIVProcedure),
    MedSpaProcedure(MedSpaProcedure),
    MemoToRecord(MemoToRecord),
    NurseVisit(NurseVisit),
    OBEvaluation(OBEvaluation),
    OfficeForm(OfficeForm),
    Phone(Phone),
    PhysicalTherapyDischargeSummary(PhysicalTherapyDischargeSummary),
    PhysicalTherapyInitialEvaluation(PhysicalTherapyInitialEvaluation),
    PhysicalTherapyInterim(PhysicalTherapyInterim),
    PhysicalTherapyProgress(PhysicalTherapyProgress),
    Procedure(Procedure),
    PsychInitialVisit(PsychInitialVisit),
    PsychProgress(PsychProgress),
    SpeechAndLanguageInitialEvaluation(SpeechAndLanguageInitialEvaluation),
    SpeechAndLanguageProgressReport(SpeechAndLanguageProgressReport),
    SpeechAndLanguageTreatmentNote(SpeechAndLanguageTreatmentNote),
    TelehealthHnP(TelehealthHnP),
    TelehealthSOAP(TelehealthSOAP),
    TherapistInitialVisit(TherapistInitialVisit),
    TherapistProgress(TherapistProgress),
    UrgentCare(UrgentCare),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export, export_to = "../../bindings/HistoryAndPhysical.ts")]
pub struct HistoryAndPhysical {
    pub chief_complaint: String,
    pub history_of_present_illness: String,
    pub past_medical_history: String,
    pub past_surgical_history: String,
    pub family_history: String,
    pub social_history: String,
    pub obstetric_and_pregnancy_history: String,
    pub hospitalizations: String,
    pub implantable_devices: String,
    pub review_of_systems: String,
    pub medications: Vec<MedicationsState>,
    pub allergies: Vec<AllergiesState>,
    pub mental_or_functional: String,
    pub vitals: VitalsState,
    pub exam: String,
    pub assessment: String,
    pub plan: String,
    pub minor_procedures: String,
    pub goals: String,
    pub health_concerns: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/SOAPNote.ts")]
pub struct SOAPNote {
    pub chief_complaint: String,
    pub subjective: String,
    pub medications: Vec<MedicationsState>,
    pub allergies: Vec<AllergiesState>,
    pub mental_or_functional: String,
    pub vitals: VitalsState,
    pub objective: String,
    pub assessment: String,
    pub plan: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/AcupunctureFollowUp.ts")]
pub struct AcupunctureFollowUp {
    pub chief_complaint: String,
    pub subjective: String,
    pub medications: Vec<MedicationsState>,
    pub allergies: Vec<AllergiesState>,
    pub objective: String,
    pub tcm_exam: String,
    pub assessment: String,
    pub plan: String,
    pub treatment: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/AcupunctureInitialVisit.ts")]
pub struct AcupunctureInitialVisit {
    cc: String,
    subjective: String,
    medications: Vec<String>,
    allergies: Vec<String>,
    objective: String,
    tcm_exam: String,
    assessment: String,
    plan: String,
    treatment: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/AcupunctureReassessment.ts")]
pub struct AcupunctureReassessment {
    cc: String,
    subjective: String,
    medications: Vec<String>,
    allergies: Vec<String>,
    objective: String,
    tcm_exam: String,
    assessment: String,
    plan: String,
    treatment: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/Amendment.ts")]
pub struct Amendment {
    pub source_of_request: String,
    pub request_details: String,
    pub decision: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/Consultation.ts")]
pub struct Consultation {
    cc: String,
    hpi: String,
    pmhx: Vec<String>,
    pshx: Vec<String>,
    fhx: Vec<String>,
    soc_hx: Vec<String>,
    ob_preg_hx: Vec<String>,
    hospitalizations: Vec<String>,
    implantable_devices: Vec<String>,
    ros: String,
    medications: Vec<String>,
    allergies: Vec<String>,
    mental_functional: String,
    vitals: String,
    exam: String,
    assessment: String,
    plan: String,
    minor_procedures: String,
    goals: String,
    health_concerns: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/DischargeSummary.ts")]
pub struct DischargeSummary {
    cc: String,
    hpi: String,
    pmhx: Vec<String>,
    pshx: Vec<String>,
    fhx: Vec<String>,
    soc_hx: Vec<String>,
    ob_preg_hx: Vec<String>,
    hospitalizations: Vec<String>,
    implantable_devices: Vec<String>,
    ros: String,
    medications: Vec<String>,
    allergies: Vec<String>,
    mental_functional: String,
    vitals: String,
    exam: String,
    assessment: String,
    plan: String,
    minor_procedures: String,
    goals: String,
    health_concerns: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/Group.ts")]
pub struct Group {
    pub group_session_content: String,
    pub session_details: String,
    pub individual_behavior_during_session: String,
    pub dsm_5: String,
    pub assessment: String,
    pub plan: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/MedSpaIVProcedure.ts")]
pub struct MedSpaIVProcedure {
    procedure: String,
    performed_by: String,
    indication: String,
    comments: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/MedSpaProcedure.ts")]
pub struct MedSpaProcedure {
    pub procedure: String,
    pub performed_by: String,
    pub indication: String,
    pub comments: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/MemoToRecord.ts")]
pub struct MemoToRecord {
    memo: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/NurseVisit.ts")]
pub struct NurseVisit {
    hpi: String,
    pmhx: Vec<String>,
    pshx: Vec<String>,
    fhx: Vec<String>,
    soc_hx: Vec<String>,
    ob_preg_hx: Vec<String>,
    hospitalizations: Vec<String>,
    medications: Vec<String>,
    allergies: Vec<String>,
    vitals: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/OBEvaluation.ts")]
pub struct OBEvaluation {
    cc: String,
    hpi: String,
    pmhx: Vec<String>,
    pshx: Vec<String>,
    fhx: Vec<String>,
    soc_hx: Vec<String>,
    ob_preg_hx: Vec<String>,
    past_pregnancy_hx: String,
    hospitalizations: Vec<String>,
    medications: Vec<String>,
    allergies: Vec<String>,
    mental_functional: String,
    vitals: String,
    exam: String,
    gestational_age: String,
    ob_exam: String,
    past_lab_hx: String,
    assessment: String,
    plan: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/OfficeForm.ts")]
pub struct OfficeForm {
    pub form: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/Phone.ts")]
pub struct Phone {
    pub discussion: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(
    export,
    export_to = "../../bindings/PhysicalTherapyDischargeSummary.ts"
)]
pub struct PhysicalTherapyDischargeSummary {
    reason_for_referral: String,
    subjective: String,
    objective: String,
    tests_and_measures: String,
    treatment: String,
    assessment: String,
    plan: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(
    export,
    export_to = "../../bindings/PhysicalTherapyInitialEvaluation.ts"
)]
pub struct PhysicalTherapyInitialEvaluation {
    reason_for_referral: String,
    subjective: String,
    medications: Vec<String>,
    vitals: String,
    objective: String,
    tests_and_measures: String,
    treatment: String,
    assessment: String,
    plan: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/PhysicalTherapyInterim.ts")]
pub struct PhysicalTherapyInterim {
    reason_for_referral: String,
    subjective: String,
    medications: Vec<String>,
    vitals: String,
    objective: String,
    tests_and_measures: String,
    treatment: String,
    assessment: String,
    plan: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/PhysicalTherapyProgress.ts")]
pub struct PhysicalTherapyProgress {
    reason_for_referral: String,
    subjective: String,
    medications: Vec<String>,
    vitals: String,
    objective: String,
    tests_and_measures: String,
    treatment: String,
    assessment: String,
    plan: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/Procedure.ts")]
pub struct Procedure {
    procedure: String,
    performed_by: String,
    indication: String,
    comments: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/PsychInitialVisit.ts")]
pub struct PsychInitialVisit {
    cc: String,
    hpi: String,
    psych_hx: String,
    pmhx: Vec<String>,
    pshx: Vec<String>,
    psychfhx: String,
    psychshx: String,
    medications: Vec<String>,
    mse: String,
    tests: String,
    psych_impression: String,
    dsm_5: String,
    assessment: String,
    plan: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/PsychProgress.ts")]
pub struct PsychProgress {
    cc: String,
    psych_symptom_follow_up: String,
    psych_syndromes: String,
    medications: Vec<String>,
    mse: String,
    psych_impression: String,
    dsm_5: String,
    psych_intervention: String,
    assessment: String,
    plan: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(
    export,
    export_to = "../../bindings/SpeechAndLanguageInitialEvaluation.ts"
)]
pub struct SpeechAndLanguageInitialEvaluation {
    cc: String,
    history: String,
    speech_development: String,
    receptive_expressive_language: String,
    pragmatics: String,
    oral_mechanism_exam: String,
    articulation: String,
    fluency: String,
    voice: String,
    assessment: String,
    plan: String,
    goals: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(
    export,
    export_to = "../../bindings/SpeechAndLanguageProgressReport.ts"
)]
pub struct SpeechAndLanguageProgressReport {
    cc: String,
    subjective: String,
    current_goals: String,
    receptive_expressive_language: String,
    pragmatics: String,
    oral_mechanism_exam: String,
    articulation: String,
    fluency: String,
    voice: String,
    assessment: String,
    plan: String,
    goals: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/SpeechAndLanguageTreatmentNote.ts")]
pub struct SpeechAndLanguageTreatmentNote {
    cc: String,
    subjective: String,
    objective: String,
    assessment: String,
    plan: String,
    goals: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/TelehealthHnP.ts")]
pub struct TelehealthHnP {
    cc: String,
    hpi: String,
    pmhx: Vec<String>,
    pshx: Vec<String>,
    fhx: Vec<String>,
    soc_hx: Vec<String>,
    ob_preg_hx: Vec<String>,
    hospitalizations: Vec<String>,
    implantable_devices: Vec<String>,
    ros: String,
    medications: Vec<String>,
    allergies: Vec<String>,
    mental_functional: String,
    vitals: String,
    exam: String,
    assessment: String,
    plan: String,
    goals: String,
    health_concerns: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/TelehealthSOAP.ts")]
pub struct TelehealthSOAP {
    cc: String,
    subjective: String,
    medications: Vec<String>,
    allergies: Vec<String>,
    mental_functional: String,
    vitals: String,
    objective: String,
    assessment: String,
    plan: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/TherapistInitialVisit.ts")]
pub struct TherapistInitialVisit {
    cc: String,
    hpi: String,
    psych_hx: String,
    psychfhx: String,
    psychshx: String,
    medications: Vec<String>,
    mse: String,
    tests: String,
    psych_impression: String,
    dsm_5: String,
    assessment: String,
    plan: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/TherapistProgress.ts")]
pub struct TherapistProgress {
    cc: String,
    psych_symptom_follow_up: String,
    psych_syndromes: String,
    medications: Vec<String>,
    mse: String,
    psych_impression: String,
    dsm_5: String,
    psych_intervention: String,
    assessment: String,
    plan: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/UrgentCare.ts")]
pub struct UrgentCare {
    cc: String,
    hpi: String,
    pmhx: Vec<String>,
    pshx: Vec<String>,
    fhx: Vec<String>,
    soc_hx: Vec<String>,
    hospitalizations: Vec<String>,
    ros: String,
    medications: Vec<String>,
    allergies: Vec<String>,
    vitals: String,
    exam: String,
    diagnostic_studies: String,
    medical_decision_making: String,
    assessment: String,
    plan: String,
    minor_procedures: String,
}

#[derive(Clone, Debug, Default, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/CurrentNoteState.ts")]
pub enum CurrentNoteState {
    #[default]
    Open,
    Signed,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/Note1.ts")]
pub struct NoteAdd {
    pub patient_id: String,
    pub note: NoteType,
    pub note_state: CurrentNoteState,
    pub last_updated_input: LastUpdatedInput,
}
