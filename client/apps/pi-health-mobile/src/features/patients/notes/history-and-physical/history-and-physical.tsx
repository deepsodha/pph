import { Box, Center, Fade } from "@chakra-ui/react";
import { db } from "../../../../db/db";
import Add from "../../../../assets/Plus Icon.svg";
import { useLiveQuery } from "dexie-react-hooks";
import { AddHistoryAndPhysicalDataFn } from "../../../../query-mutation-services/history-and-physical-data-fns";
import HistoryAndPhysicalForm from "../../../../../../../packages/ui/forms/history-and-physical-form";
import type { HistoryAndPhysicalAdd } from "@repo/types/dto";
import { useNavigate, useParams } from "@tanstack/react-router";
import { useAtom } from "jotai";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
} from "../../../../atoms/header";
import { useMountEffect } from "@react-hookz/web";

const HistoryAndPhysical = () => {
	const navigate = useNavigate();
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);

	const addMutation = AddHistoryAndPhysicalDataFn();
	const vitalsData = useLiveQuery(() => db.vitals.toArray());
	const allergyData = useLiveQuery(() => db.allergy.toArray());
	const medicationData = useLiveQuery(() => db.medications.toArray());

	const patientAddId = useParams({
		from: "/notes/add/$patientId",
		select: (params) => params.patientId,
	});
	console.log(patientAddId, "addId");

	// const handpId =
	// 	useParams({
	// 		from: "/notes/edit/$patientId/$notesId",
	// 		select: (params) => params.notesId,
	// 	});

	// const HandPData = useLiveQuery(() => db.historyandphysical.toArray());
	// const filteredData = HandPData?.find((item) => item.id === handpId);

	const vitalFilterData = vitalsData?.filter(
		(item) => item.patient_id === patientAddId,
	);
	const allergyFilterData = allergyData?.filter(
		(item) => item.patient_id === patientAddId,
	);
	const medicationFilterData = medicationData?.filter(
		(item) => item.patient_id === patientAddId,
	);

	useMountEffect(() => {
		setHeaderText("Add HistoryAndPhysical");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
	});

	return (
		<div>
			<Fade in={true}>
				<Box minHeight={"100vh"} bgColor={"#F7F7F9"} overflowY="auto">
					<Center>
						<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
							<HistoryAndPhysicalForm
								onSubmit={(p) => {
									addMutation.mutateAsync(p as HistoryAndPhysicalAdd);

									console.log(p);
								}}
								patientId={patientAddId}
								// initialValues={filteredData}
								lastUpdatedInput={db.getLastUpdated}
								addicon={Add}
								// edit={role === "edit"}
								// HandPId={handpId}
								vitals={vitalFilterData}
								medication={medicationFilterData}
								allergy={allergyFilterData}
								Link={(path) => navigate({ to: path })}
							/>
						</Box>
					</Center>
				</Box>
			</Fade>
		</div>
	);
};

export default HistoryAndPhysical;
