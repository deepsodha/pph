import { useMountEffect } from "@react-hookz/web";
import { useAtom } from "jotai";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
} from "../../../atoms/header";
import { Box, Center, Fade } from "@chakra-ui/react";
import { HeadingTag } from "../../../components/heading-tag";
import { HospitalizationForm } from "@repo/ui/forms";
import type {
	HospitalizationAdd,
	HospitalizationState,
	HospitalizationUpdate,
} from "@repo/types/dto";
import { useParams, useRouterState } from "@tanstack/react-router";
import { db } from "../../../db/db";
import { useLiveQuery } from "dexie-react-hooks";
import {
	AddHospitalizationDataFn,
	UpdateHospitalizationDataFn,
} from "../../../query-mutation-services/hospitalization-data-fn";
const hospitalization = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const router = useRouterState();
	const role = router.location.pathname.split("/")[2];
	const { patientId } = useParams({ from: "patientId" });
	const { hospitalizationId } = useParams({ from: "hospitalizationId" });
	const hospitalData = useLiveQuery(() => db.hospitalization.toArray());
	const headerData = useLiveQuery(() => db.patients.toArray());
	const headerFilterData = headerData?.find((item) => item.id === patientId);
	const filteredData = hospitalData?.find(
		(item) => item.id === hospitalizationId,
	) as HospitalizationState | undefined;
	const addMutation = AddHospitalizationDataFn();
	const updateMutation = UpdateHospitalizationDataFn();
	useMountEffect(() => {
		setHeaderText(
			role === "edit" ? "Edit Hospitalization" : "Add New Hospitalization",
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
							<HospitalizationForm
								onSubmit={(p) => {
									if (role === "edit") {
										const editVal: HospitalizationUpdate = {
											input: { ...(p as HospitalizationAdd) },
											id: hospitalizationId,
										};
										updateMutation.mutateAsync(editVal);
									} else {
										addMutation.mutateAsync(p as HospitalizationAdd);
									}
								}}
								patientId={patientId}
								hospitalizationId={hospitalizationId}
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

export default hospitalization;
