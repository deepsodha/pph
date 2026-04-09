import { Box, Center, Fade } from "@chakra-ui/react";
import SoapForm from "../../../../../../../packages/ui/forms/soap-form";
import { db } from "../../../../db/db";
import Add from "../../../../assets/Plus Icon.svg";
import { AddSoapDataFn } from "../../../../query-mutation-services/soap-data-fn";
import { useLiveQuery } from "dexie-react-hooks";
import type { SoapAdd } from "@repo/types/dto";
import { useMountEffect } from "@react-hookz/web";
import { useParams, useNavigate } from "@tanstack/react-router";
import { useAtom } from "jotai";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
} from "../../../../atoms/header";
const Soap = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const navigate = useNavigate();
	const addMutation = AddSoapDataFn();

	const vitalsData = useLiveQuery(() => db.vitals.toArray());
	const allergyData = useLiveQuery(() => db.allergy.toArray());
	const medicationData = useLiveQuery(() => db.medications.toArray());

	const patientAddId = useParams({
		from: "/notes/add/$patientId",
		select: (params) => params.patientId,
	});

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
		setHeaderText("Add Soap");
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
							<SoapForm
								onSubmit={(p) => {
									addMutation.mutateAsync(p as SoapAdd);
									console.log(p);
								}}
								patient={patientAddId}
								lastUpdatedInput={db.getLastUpdated}
								addicon={Add}
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
export default Soap;
