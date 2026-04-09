import { Fade, Box, Center } from "@chakra-ui/react";
import { useMountEffect } from "@react-hookz/web";
import type {
	FamilyHistoryAdd,
	FamilyHistoryState,
	FamilyHistoryUpdate,
} from "@repo/types/dto";
import { useRouterState, useParams } from "@tanstack/react-router";
import { useLiveQuery } from "dexie-react-hooks";
import { useAtom } from "jotai";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
} from "../../../../atoms/header";
import { HeadingTag } from "../../../../components/heading-tag";
import { db } from "../../../../db/db";
import { FamilyHistoryForm } from "@repo/ui/forms";
import {
	AddFamilyHistoryDataFn,
	UpdateFamilyHistoryDataFn,
} from "../../../../query-mutation-services/family-history-data-fn";

const familyhistory = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const addMutation = AddFamilyHistoryDataFn();
	const updateMutation = UpdateFamilyHistoryDataFn();
	const router = useRouterState();
	const role = router.location.pathname.split("/")[2];
	const { patientId } = useParams({ from: "patientId" });
	const { FamilyHistoryId } = useParams({ from: "FamilyHistoryId" });
	const familyHistoryData = useLiveQuery(() => db.familyhistory.toArray());
	const filteredData = familyHistoryData?.find(
		(item) => item.id === FamilyHistoryId,
	) as FamilyHistoryState | undefined;

	const headerData = useLiveQuery(() => db.patients.toArray());
	const headerFilterData = headerData?.find((item) => item.id === patientId);
	useMountEffect(() => {
		setHeaderText(
			role === "edit" ? "Edit Family History" : "Add New Family History",
		);
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
							<HeadingTag
								label_text={
									headerFilterData?.user.first_name || "Default Value"
								}
							/>
							<FamilyHistoryForm
								onSubmit={(p) => {
									if (role === "edit") {
										const editVal: FamilyHistoryUpdate = {
											input: { ...(p as FamilyHistoryAdd) },
											id: FamilyHistoryId,
										};
										updateMutation.mutateAsync(editVal);
									} else {
										addMutation.mutateAsync(p as FamilyHistoryAdd);
									}
								}}
								patientId={patientId}
								FamilyHistoryId={FamilyHistoryId}
								initialValues={filteredData}
								edit={role === "edit"}
								lastUpdatedInput={db.getLastUpdated}
							/>
						</Box>
					</Center>
				</Box>
			</Fade>
		</div>
	);
};

export default familyhistory;
