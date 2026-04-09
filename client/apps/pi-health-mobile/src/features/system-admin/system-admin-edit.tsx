import { Box } from "@chakra-ui/react";

import { useParams } from "@tanstack/react-router";
import { useAtom } from "jotai";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
} from "../../atoms/header";
// import { SystemAdminForm } from "@repo/ui/forms";
import EyeIcon from "../../assets/confirm_password_icon.svg";
import hideEyeIcon from "../../assets/hideyeicon.svg";
import type { SystemAdminUpdate } from "@repo/types/dto";
import { db } from "../../db/db";
import { UpdateSystemAdminFn } from "../../query-mutation-services/system-admin-data-fn";
import { useLiveQuery } from "dexie-react-hooks";
import {
	useDebouncedState,
	useMountEffect,
	useUpdateEffect,
} from "@react-hookz/web";
import { SystemAdminForm } from "@repo/ui/forms";
import { motion, AnimatePresence } from "framer-motion";
import { GetOrganizationLocationPinCodeDataFn } from "../../query-mutation-services/organization-data-fn";

const MotionBox = motion(Box);
const SystemAdminEditForm = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);

	const systemAdminId = useParams({
		from: "/systemadmin/edit/$systemAdminId",
		select: (params) => params.systemAdminId,
	});

	useMountEffect(() => {
		setHeaderText("Edit SystemAdmin");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
	});
	const updateMutation = UpdateSystemAdminFn();
	const systemAdminGet = useLiveQuery(() => db.systemadmin.toArray());
	const organizationGet = useLiveQuery(() => db.organization.toArray());

	const filteredData = systemAdminGet?.find(
		(item) => item.id === systemAdminId,
	);

	const organizationId = filteredData?.org_id || "";
	const filteredData2 = organizationGet?.find(
		(item) => item.id === organizationId,
	);
	const [pincodedata, setPincodeData] = useDebouncedState("", 500);
	console.log(pincodedata, "i changed pincode data from form ");

	const locationData = GetOrganizationLocationPinCodeDataFn(pincodedata);
	console.log(locationData.data?.result);
	useUpdateEffect(() => {
		locationData.refetch();
	}, [pincodedata]);
	return (
		<div>
			<AnimatePresence>
				<Box bgColor={"#F7F7F9"}>
					<MotionBox
						initial={{ opacity: 0, x: 50 }}
						animate={{ opacity: 1, x: 0 }}
						exit={{ opacity: 0, x: 50 }}
						transition={{ duration: 0.65 }}
					>
						<SystemAdminForm
							onSubmit={(p) => {
								const editVal: SystemAdminUpdate = {
									input: { ...p },
									id: systemAdminId,
									org_id: organizationId,
								};
								updateMutation.mutateAsync(editVal);
							}}
							image_url1={hideEyeIcon}
							image_url2={EyeIcon}
							lastUpdatedInput={db.getLastUpdated}
							initialValues={filteredData}
							initialValues2={filteredData2}
							edit={true}
							locationData={locationData.data?.result}
							pincodeOnChange={(pincode) => setPincodeData(pincode)}
							//   pincode={PinCode.data?.result}
						/>
					</MotionBox>
				</Box>
			</AnimatePresence>
		</div>
	);
};

export default SystemAdminEditForm;
