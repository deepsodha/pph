import type { MedicationsState } from "@repo/types/dto";
import NotestListField from "./notes-list-field";
import { Divider } from "@chakra-ui/react";

type NotesListMedicationCompProps = {
	medication: MedicationsState;
};
const NotesListMedicationComp = ({
	medication,
}: NotesListMedicationCompProps) => {
	return (
		<div>
			<div key={medication.id}>
				<Divider border="1px solid" />
				<NotestListField label={"Drug:"} value={medication.drug} />
				<NotestListField label={"Status:"} value={medication.status} />
				<NotestListField
					label={"Instruction:"}
					value={medication.instruction}
				/>
				<NotestListField label={"Comments:"} value={medication.comments} />
			</div>
		</div>
	);
};

export default NotesListMedicationComp;
