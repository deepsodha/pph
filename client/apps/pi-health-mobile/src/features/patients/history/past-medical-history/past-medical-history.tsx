import { Box, Center, Fade } from "@chakra-ui/react";
import { useParams, useRouterState } from "@tanstack/react-router";

import { useAtom } from "jotai";
import {
	addValue,
	dashboardValue,
	formValue,
	headerText,
} from "../../../../atoms/header";
import { PastMedicalHistoryForm } from "@repo/ui/forms";
import { db } from "../../../../db/db";
import { useMountEffect } from "@react-hookz/web";
import type {
	PastMedicalHistoryAdd,
	PastMedicalHistoryState,
	PastMedicalHistoryUpdate,
} from "@repo/types/dto";
import {
	AddPastMedicalHistoryDataFn,
	UpdatePastMedicalHistoryDataFn,
} from "../../../../query-mutation-services/past-medical-history";
import { useLiveQuery } from "dexie-react-hooks";
import { HeadingTag } from "../../../../components/heading-tag";

const ServiceLocationDetails = () => {
	// const { pastMedicalHistoryId } = useParams({ from: "pastMedicalHistoryId" });
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);

	// const pastMedicalHistory = useLiveQuery(() => db.pastmedicalhistory.toArray());

	const router = useRouterState();
	const role = router.location.pathname.split("/")[2];

	const addMutation = AddPastMedicalHistoryDataFn();
	const updateMutation = UpdatePastMedicalHistoryDataFn();
	const { patientId } = useParams({ from: "patientId" });
	const { PastMedicalHistoryId } = useParams({ from: "PastMedicalHistoryId" });

	const pastMedicalHistoryData = useLiveQuery(() =>
		db.pastmedicalhistory.toArray(),
	);
	const filteredData = pastMedicalHistoryData?.find(
		(item) => item.id === PastMedicalHistoryId,
	) as PastMedicalHistoryState | undefined;
	const headerData = useLiveQuery(() => db.patients.toArray());
	const headerFilterData = headerData?.find((item) => item.id === patientId);

	useMountEffect(() => {
		setHeaderText(
			role === "edit"
				? "Edit Past Medical History"
				: "Add Past Medical History",
		);
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
	});

	return (
		<>
			<Fade in={true}>
				<Box minHeight={"100vh"} bgColor={"#F7F7F9"} overflowY="auto">
					<Center>
						<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
							<HeadingTag
								label_text={
									headerFilterData?.user.first_name || "Default Value"
								}
							/>
							<PastMedicalHistoryForm
								onSubmit={(p) => {
									if (role === "edit") {
										const editVal: PastMedicalHistoryUpdate = {
											input: { ...(p as PastMedicalHistoryAdd) },
											id: PastMedicalHistoryId,
										};
										updateMutation.mutateAsync(editVal);
									} else {
										addMutation.mutateAsync(p as PastMedicalHistoryAdd);
									}
								}}
								edit={role === "edit"}
								patientId={patientId}
								initialValues={filteredData}
								PastMedicalHistoryId={PastMedicalHistoryId}
								lastUpdatedInput={db.getLastUpdated}
							/>
						</Box>
					</Center>
				</Box>
			</Fade>
		</>
	);
};

export default ServiceLocationDetails;
