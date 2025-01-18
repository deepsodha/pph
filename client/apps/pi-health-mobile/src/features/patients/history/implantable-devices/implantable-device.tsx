import { Fade, Box, Center } from "@chakra-ui/react";
import { useMountEffect } from "@react-hookz/web";
import type {
	ImplantableDevicesAdd,
	ImplantableDevicesState,
	ImplantableDevicesUpdate,
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
import { ImplantableDeviceForm } from "@repo/ui/forms";
import {
	AddImplantableDevicesDataFn,
	UpdateImplantableDevicesDataFn,
} from "../../../../query-mutation-services/implantable-devices";

const ImplantableDevice = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const addMutation = AddImplantableDevicesDataFn();
	const updateMutation = UpdateImplantableDevicesDataFn();
	const router = useRouterState();
	const role = router.location.pathname.split("/")[2];
	const { patientId } = useParams({ from: "patientId" });
	const { ImplantabledeviceId } = useParams({ from: "ImplantabledeviceId" });
	const implantabledevicesData = useLiveQuery(() =>
		db.implantabledevices.toArray(),
	);
	const filteredData = implantabledevicesData?.find(
		(item) => item.id === ImplantabledeviceId,
	) as ImplantableDevicesState | undefined;

	const headerData = useLiveQuery(() => db.patients.toArray());
	const headerFilterData = headerData?.find((item) => item.id === patientId);
	useMountEffect(() => {
		setHeaderText(
			role === "edit" ? "Edit Implantable Device" : "Add Implantable Devices",
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
							<ImplantableDeviceForm
								onSubmit={(p) => {
									if (role === "edit") {
										const editVal: ImplantableDevicesUpdate = {
											input: { ...(p as ImplantableDevicesAdd) },
											id: ImplantabledeviceId,
										};
										updateMutation.mutateAsync(editVal);
									} else {
										addMutation.mutateAsync(p as ImplantableDevicesAdd);
									}
								}}
								patientId={patientId}
								ImplantabledeviceId={ImplantabledeviceId}
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

export default ImplantableDevice;
