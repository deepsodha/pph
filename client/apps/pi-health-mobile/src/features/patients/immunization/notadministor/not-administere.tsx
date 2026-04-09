import { Box, Center, Fade } from "@chakra-ui/react";
import {
	AddNotAdministerDataFn,
	UpdateNotAdministerDataFn,
} from "../../../../query-mutation-services/not-administer-data-fn";
import { useParams, useRouterState } from "@tanstack/react-router";
import { useAtom } from "jotai";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
} from "../../../../atoms/header";
import { NotAdministeredForm } from "@repo/ui/forms";
import type {
	NotAdministeredAdd,
	NotAdministeredUpdate,
} from "@repo/types/dto";
import { db } from "../../../../db/db";
import { useLiveQuery } from "dexie-react-hooks";
import { useMountEffect } from "@react-hookz/web";
import { VaccineData } from "../../../../atoms/vaccine-data";

const NotAdministered = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const addMutation = AddNotAdministerDataFn();
	const updateMutation = UpdateNotAdministerDataFn();
	const { vaccine } = VaccineData();
	const router = useRouterState();
	const role = router.location.pathname.split("/")[2];
	const { patientId } = useParams({ from: "patientId" });
	const { notAdministerId } = useParams({ from: "notAdministerId" });

	const notAdminister = useLiveQuery(() => db.notadminister.toArray());
	const filteredData = notAdminister?.find(
		(item) => item.id === notAdministerId,
	);

	useMountEffect(() => {
		setHeaderText(
			role === "edit" ? "Edit Not Administer" : "Add New Not Administer",
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
							<NotAdministeredForm
								onSubmit={(p) => {
									if (role === "edit") {
										const editVal: NotAdministeredUpdate = {
											id: notAdministerId,
											input: { ...(p as NotAdministeredAdd) },
										};
										updateMutation.mutateAsync(editVal);
									} else {
										addMutation.mutateAsync(p as NotAdministeredAdd);
									}
								}}
								patientId={patientId}
								initialValues={filteredData}
								lastUpdatedInput={db.getLastUpdated}
								edit={role === "edit"}
								vaccine={vaccine}
							/>
						</Box>
					</Center>
				</Box>
			</Fade>
		</div>
	);
};

export default NotAdministered;
