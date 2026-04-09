import { Divider } from "@chakra-ui/react";
import type { VitalsState } from "@repo/types/dto";
import NotestListField from "./notes-list-field";

type NotesListVitalsCompProps = {
	vitals: VitalsState;
};
const NotesListVitalsComp = ({ vitals }: NotesListVitalsCompProps) => {
	return (
		<div>
			<Divider border="1px solid" />
			<NotestListField label={"Temperature:"} value={vitals.temperature} />
			<NotestListField label={"BP:"} value={vitals.blood_pressure} />
			<NotestListField label={"BMI:"} value={vitals.bmi} />
			<NotestListField label={"Heart Rate:"} value={vitals.heart_rate} />
			<NotestListField label={"Height:"} value={vitals.height} />
			<NotestListField label={"Weight:"} value={vitals.weight} />
			<NotestListField label={"Comments:"} value={vitals.comments} />
		</div>
	);
};

export default NotesListVitalsComp;
