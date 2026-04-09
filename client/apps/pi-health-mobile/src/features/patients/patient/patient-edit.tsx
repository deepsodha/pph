import { Box, useUpdateEffect } from "@chakra-ui/react";
import { UpdatePatientFn } from "../../../query-mutation-services/patient-data-fn";
import { useParams } from "@tanstack/react-router";
import { useAtom } from "jotai";
import {
	addValue,
	dashboardValue,
	displayMenu,
	formValue,
	headerText,
} from "../../../atoms/header";
import type { PatientAdd, PatientUpdate } from "@repo/types/dto";
import { PatientForm } from "@repo/ui/forms";
import { db } from "../../../db/db";
import { useLiveQuery } from "dexie-react-hooks";
import { useDebouncedState, useMountEffect } from "@react-hookz/web";
import { motion, AnimatePresence } from "framer-motion";
import { GetOrganizationLocationPinCodeDataFn } from "../../../query-mutation-services/organization-data-fn";

const MotionBox = motion(Box);
const PatientEditForm = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const [, setmenu] = useAtom(displayMenu);
	const patientId = useParams({
		from: "/patient/edit/$patientId",
		select: (params) => params.patientId,
	});
	const updateMutation = UpdatePatientFn();
	const patientData = useLiveQuery(() => db.patients.toArray());
	const filteredData = patientData?.find((item) => item.id === patientId);
	useMountEffect(() => {
		setHeaderText("Edit Patient");
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
						initial={{ opacity: 0, x: 50 }}
						animate={{ opacity: 1, x: 0 }}
						exit={{ opacity: 0, x: 50 }}
						transition={{ duration: 0.65 }}
					>
						<PatientForm
							onSubmit={(p) => {
								const editVal: PatientUpdate = {
									id: patientId,
									input: { ...(p as PatientAdd) },
								};
								updateMutation.mutateAsync(editVal);
							}}
							patientId={patientId}
							initialValues={filteredData}
							edit={true}
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

export default PatientEditForm;
