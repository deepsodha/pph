import { Fade, Box, Center } from "@chakra-ui/react";
import { useMountEffect } from "@react-hookz/web";
import type {
	PastSurgicalHistoryAdd,
	PastSurgicalHistoryUpdate,
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
import { PastSurgicalHistoryForm } from "@repo/ui/forms";
import {
	AddPastSurgicalHistoryFn,
	UpdatePastSurgicalHistoryFn,
} from "../../../../query-mutation-services/past-surgical-history";

const pastSurgicalhistory = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const addMutation = AddPastSurgicalHistoryFn();
	const updateMutation = UpdatePastSurgicalHistoryFn();
	const router = useRouterState();
	const role = router.location.pathname.split("/")[2];
	const { patientId } = useParams({ from: "patientId" });
	const { PastSurgicalHistoryId } = useParams({
		from: "PastSurgicalHistoryId",
	});
	const pastsurgicalHistory = useLiveQuery(() =>
		db.pastsurgicalhistory.toArray(),
	);

	const filteredData = pastsurgicalHistory?.find(
		(item) => item.id === PastSurgicalHistoryId,
	);

	const headerData = useLiveQuery(() => db.patients.toArray());
	const headerFilterData = headerData?.find((item) => item.id === patientId);
	useMountEffect(() => {
		setHeaderText(
			role === "edit"
				? "Edit Past Surgical History"
				: "Add New Past Surgical History",
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
							<PastSurgicalHistoryForm
								onSubmit={(p) => {
									if (role === "edit") {
										const editVal: PastSurgicalHistoryUpdate = {
											input: { ...(p as PastSurgicalHistoryAdd) },
											id: PastSurgicalHistoryId,
										};

										updateMutation.mutateAsync(editVal);
									} else {
										addMutation.mutateAsync(p as PastSurgicalHistoryAdd);
									}
								}}
								patientId={patientId}
								PastSurgicalHistoryId={PastSurgicalHistoryId}
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

export default pastSurgicalhistory;
