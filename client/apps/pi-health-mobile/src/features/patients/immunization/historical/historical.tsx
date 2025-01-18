import { Box, Center, Fade } from "@chakra-ui/react";
import {
	AddHistoricalDataFn,
	UpdateHistoricalDataFn,
} from "../../../../query-mutation-services/historical-data-fn";
import { useAtom } from "jotai";
import { useParams, useRouterState } from "@tanstack/react-router";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
} from "../../../../atoms/header";
import { HistoricalForm } from "@repo/ui/forms";
import type { HistoricalAdd, HistoricalUpdate } from "@repo/types/dto";
import { db } from "../../../../db/db";
import { useLiveQuery } from "dexie-react-hooks";
import { useMountEffect } from "@react-hookz/web";

import { VaccineData } from "../../../../atoms/vaccine-data";
const Historical = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const router = useRouterState();
	const role = router.location.pathname.split("/")[2];
	const { historicalId } = useParams({ from: "historicalId" });
	const addMutation = AddHistoricalDataFn();
	const historicalData = useLiveQuery(() => db.historical.toArray());
	const updateMutation = UpdateHistoricalDataFn();
	const { vaccine } = VaccineData();
	const filteredData = historicalData?.find((item) => item.id === historicalId);
	const { patientId } = useParams({ from: "patientId" });

	useMountEffect(() => {
		setHeaderText(role === "edit" ? "Edit Historical" : "Add New Historical");
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
							<HistoricalForm
								onSubmit={(p) => {
									if (role === "edit") {
										const editVal: HistoricalUpdate = {
											id: historicalId,
											input: { ...(p as HistoricalAdd) },
										};
										updateMutation.mutateAsync(editVal);
									} else {
										addMutation.mutateAsync(p as HistoricalAdd);
									}
								}}
								initialValues={filteredData}
								lastUpdatedInput={db.getLastUpdated}
								edit={role === "edit"}
								vaccine={vaccine}
								patientId={patientId}
							/>
						</Box>
					</Center>
				</Box>
			</Fade>
		</div>
	);
};

export default Historical;
