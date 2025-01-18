import type { LastUpdatedInput } from "./LastUpdatedInput";

export type PastMedicalHistoryAdd = {
	patient_id: string;
	blood_type: string | null;
	head: string | null;
	respiratory: string | null;
	musculoskeletal: string | null;
	endocrine: string | null;
	eyes: string | null;
	gastrointestinal: string | null;
	skin: string | null;
	ears: string | null;
	noses: string | null;
	neurological: string | null;
	heme: string | null;
	mouth: string | null;
	infectious: string | null;
	cardiovascular: string | null;
	genitourinary: string | null;
	psychiatric: string | null;
	comments: string | null;
	last_updated_input: LastUpdatedInput;
};
