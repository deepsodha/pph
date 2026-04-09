import { Box, Fade } from "@chakra-ui/react";
import { useParams, useRouterState } from "@tanstack/react-router";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
} from "../../../../atoms/header";
import { useAtom } from "jotai";
import { AdministerForm } from "@repo/ui/forms";
import {
	AddAdministerDataFn,
	UpdateAdministerDataFn,
} from "../../../../query-mutation-services/administer-data-fn";
import { useLiveQuery } from "dexie-react-hooks";
import type { AdministerUpdate, AdministerAdd } from "@repo/types/dto";
import { db } from "../../../../db/db";
import { useMountEffect } from "@react-hookz/web";
import { VaccineData } from "../../../../atoms/vaccine-data";

const Administer = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const router = useRouterState();
	const role = router.location.pathname.split("/")[2];

	const { patientId } = useParams({ from: "patientId" });
	const { administerId } = useParams({ from: "administerId" });
	const addMutation = AddAdministerDataFn();
	const administer = useLiveQuery(() => db.administer.toArray());
	const ServiceLocationData = useLiveQuery(() => db.servicelocation.toArray());
	const updateMutation = UpdateAdministerDataFn();
	const { vaccine } = VaccineData();
	const filteredData = administer?.find((item) => item.id === administerId);

	const serviceLocationList = ServiceLocationData?.map(
		({ service_location_name }) => service_location_name,
	);

	useMountEffect(() => {
		setHeaderText(role === "edit" ? "Edit Administer" : "Add New Administer");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
	});
	return (
		<div>
			<Fade in={true}>
				<Box minHeight={"100vh"} bgColor={"#F7F7F9"} overflowY="auto">
					<Box>
						<AdministerForm
							onSubmit={(p) => {
								if (role === "edit") {
									const editVal: AdministerUpdate = {
										id: administerId,
										input: { ...(p as AdministerAdd) },
									};
									updateMutation.mutateAsync(editVal);
								} else {
									addMutation.mutateAsync(p as AdministerAdd);
								}
							}}
							patientId={patientId}
							initialValues={filteredData}
							lastUpdatedInput={db.getLastUpdated}
							edit={role === "edit"}
							serviceLocationList={serviceLocationList}
							vaccine={vaccine}
						/>
					</Box>
				</Box>
			</Fade>
		</div>
	);
};

export default Administer;
