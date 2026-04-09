import { Box, useUpdateEffect } from "@chakra-ui/react";
import type { UserAdd } from "@repo/types/dto";
import { UserForm } from "@repo/ui/forms";
import { db } from "../../db/db";
import EyeIcon from "../../assets/confirm_password_icon.svg";
import hideEyeIcon from "../../assets/hideyeicon.svg";
import { useAtom } from "jotai";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
	displayMenu,
} from "../../atoms/header";
import { useDebouncedState, useMountEffect } from "@react-hookz/web";
import { AddUserDataFn } from "../../query-mutation-services/user-data-fn";
import { motion, AnimatePresence } from "framer-motion";
import { GetOrganizationLocationPinCodeDataFn } from "../../query-mutation-services/organization-data-fn";

const MotionBox = motion(Box);
const UserAddForm = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const [, setmenu] = useAtom(displayMenu);

	useMountEffect(() => {
		setHeaderText("Add New User");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
		setmenu(false);
	});
	const addMutation = AddUserDataFn();
	const [pincodedata, setPincodeData] = useDebouncedState("", 500);
	console.log(pincodedata, "i changed pincode data from form ");

	const locationData = GetOrganizationLocationPinCodeDataFn(pincodedata);
	console.log(locationData.data?.result);
	useUpdateEffect(() => {
		locationData.refetch();
	}, [pincodedata]);
	return (
		<>
			<AnimatePresence>
				<Box bgColor={"#F7F7F9"}>
					<MotionBox
						initial={{ opacity: 0, x: -50 }}
						animate={{ opacity: 1, x: 0 }}
						exit={{ opacity: 0, x: -50 }}
						transition={{ duration: 0.65 }}
					>
						<UserForm
							onSubmit={(p) => {
								addMutation.mutateAsync(p as UserAdd);
							}}
							lastUpdatedInput={db.getLastUpdated}
							edit={false}
							image_url1={hideEyeIcon}
							image_url2={EyeIcon}
							locationData={locationData.data?.result}
							pincodeOnChange={(pincode) => setPincodeData(pincode)}
						/>
					</MotionBox>
				</Box>
			</AnimatePresence>
		</>
	);
};

export default UserAddForm;
