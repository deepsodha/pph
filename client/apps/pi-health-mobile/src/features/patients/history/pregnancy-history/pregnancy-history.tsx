import { Box, Center, Fade } from "@chakra-ui/react";
import { useMountEffect } from "@react-hookz/web";
import type {
	OBandPregnancyState,
	OBandPregnancyUpdate,
	OBandPregnancyAdd,
} from "@repo/types/dto";
import { PregnancyHistoryForm } from "@repo/ui/forms";
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
import {
	AddObAndPregnancyFn,
	UpdateObAndPregnancyFn,
} from "../../../../query-mutation-services/ob-and-pregnancy";

const pregnancyHistory = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const addMutation = AddObAndPregnancyFn();
	const updateMutation = UpdateObAndPregnancyFn();
	const router = useRouterState();
	const role = router.location.pathname.split("/")[2];
	const { patientId } = useParams({ from: "patientId" });
	const { pregnancyHistoryId } = useParams({ from: "pregnancyHistoryId" });
	const preganancyHistoryData = useLiveQuery(() =>
		db.obandpregnanacy.toArray(),
	);
	const filteredData = preganancyHistoryData?.find(
		(item) => item.id === pregnancyHistoryId,
	) as OBandPregnancyState | undefined;

	const headerData = useLiveQuery(() => db.patients.toArray());
	const headerFilterData = headerData?.find((item) => item.id === patientId);
	useMountEffect(() => {
		setHeaderText(
			role === "edit"
				? "Edit OB & Pregnancy History"
				: "Add New OB & Pregnancy History",
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
							<PregnancyHistoryForm
								onSubmit={(p) => {
									if (role === "edit") {
										const editVal: OBandPregnancyUpdate = {
											input: { ...(p as OBandPregnancyAdd) },
											id: pregnancyHistoryId,
										};
										updateMutation.mutateAsync(editVal);
									} else {
										addMutation.mutateAsync(p as OBandPregnancyAdd);
									}
								}}
								patientId={patientId}
								pregnancyHistoryId={pregnancyHistoryId}
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

export default pregnancyHistory;
