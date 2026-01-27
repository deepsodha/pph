import { Box } from "@chakra-ui/react";
import { useAtom } from "jotai";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
} from "../../atoms/header";
import EyeIcon from "../../assets/confirm_password_icon.svg";
import hideEyeIcon from "../../assets/hideyeicon.svg";
import type { SystemAdminAdd } from "@repo/types/dto";
import { db } from "../../db/db";
import { AddSystemAdminDataFn } from "../../query-mutation-services/system-admin-data-fn";
import {
	useDebouncedState,
	useMountEffect,
	useUpdateEffect,
} from "@react-hookz/web";
import { SystemAdminForm } from "@repo/ui/forms";
import { motion, AnimatePresence } from "framer-motion";
import { GetOrganizationLocationPinCodeDataFn } from "../../query-mutation-services/organization-data-fn";

const MotionBox = motion(Box);
const SystemAdminAddForm = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	// const {pincode} = PincodeData()

	useMountEffect(() => {
		setHeaderText("Add New SystemAdmin");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
	});
	const addMutation = AddSystemAdminDataFn();
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
						initial={{ opacity: 0, x: -50 }}
						animate={{ opacity: 1, x: 0 }}
						exit={{ opacity: 0, x: -50 }}
						transition={{ duration: 0.65 }}
					>
						<SystemAdminForm
							onSubmit={(p) => {
								addMutation.mutateAsync(p as SystemAdminAdd);
								console.log(p);
							}}
							image_url1={hideEyeIcon}
							image_url2={EyeIcon}
							lastUpdatedInput={db.getLastUpdated}
							edit={false}
							locationData={locationData.data?.result}
							pincodeOnChange={(pincode) => setPincodeData(pincode)}
						/>
					</MotionBox>
				</Box>
			</AnimatePresence>
		</div>
	);
};

export default SystemAdminAddForm;
