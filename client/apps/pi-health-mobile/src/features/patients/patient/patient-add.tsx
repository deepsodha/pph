import { Box, useUpdateEffect } from "@chakra-ui/react";
import { AddPatientDataFn } from "../../../query-mutation-services/patient-data-fn";
import { useAtom } from "jotai";
import {
	addValue,
	dashboardValue,
	displayMenu,
	formValue,
	headerText,
} from "../../../atoms/header";
import type { PatientAdd } from "@repo/types/dto";
import { PatientForm } from "@repo/ui/forms";
import { db } from "../../../db/db";
import { useDebouncedState, useMountEffect } from "@react-hookz/web";
import { motion, AnimatePresence } from "framer-motion";
import { GetOrganizationLocationPinCodeDataFn } from "../../../query-mutation-services/organization-data-fn";

const MotionBox = motion(Box);
const PatientAddForm = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const [, setmenu] = useAtom(displayMenu);

	const addMutation = AddPatientDataFn();
	useMountEffect(() => {
		setHeaderText("Add New Patient");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
		setmenu(false);
	});

	const [pincodedata, setPincodeData] = useDebouncedState("", 500);
	console.log(pincodedata, "i changed pincode data from form ");

	const locationData = GetOrganizationLocationPinCodeDataFn(pincodedata);
	console.log(locationData.data?.result);
	useUpdateEffect(() => {
		locationData.refetch();
	}, [pincodedata]);

	return (
		<>
			<AnimatePresence>
				<Box bgColor={"#F7F7F9"}>
					<MotionBox
						initial={{ opacity: 0, x: -50 }}
						animate={{ opacity: 1, x: 0 }}
						exit={{ opacity: 0, x: -50 }}
						transition={{ duration: 0.65 }}
					>
						<PatientForm
							onSubmit={(p) => {
								addMutation.mutateAsync(p as PatientAdd);
							}}
							edit={false}
							lastUpdatedInput={db.getLastUpdated}
							locationData={locationData.data?.result}
							pincodeOnChange={(pincode) => setPincodeData(pincode)}
						/>
					</MotionBox>
				</Box>
			</AnimatePresence>
		</>
	);
};

export default PatientAddForm;
