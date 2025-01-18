import { Divider } from "@chakra-ui/react";
import type { AllergiesState } from "@repo/types/dto";
import NotestListField from "./notes-list-field";

type NotesListAllergyCompProps = {
	allergy: AllergiesState;
};
const NotesListAllergyComp = ({ allergy }: NotesListAllergyCompProps) => {
	return (
		<div>
			<div key={allergy.id}>
				<Divider border="1px solid" />
				<NotestListField label={"Allergy name:"} value={allergy.allergen} />
				<NotestListField
					label={"Severities:"}
					value={allergy.allergy_severities}
				/>
				<NotestListField label={"Reaction:"} value={allergy.reaction} />
				<NotestListField label={"Status:"} value={allergy.stauts} />
				<NotestListField label={"Comments:"} value={allergy.comments} />
			</div>
		</div>
	);
};

export default NotesListAllergyComp;
