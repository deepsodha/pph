import { Box, useUpdateEffect } from "@chakra-ui/react";
import type { UserUpdate } from "@repo/types/dto";
import { UserForm } from "@repo/ui/forms";
import { useParams } from "@tanstack/react-router";
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
import { UpdateUserDataFn } from "../../query-mutation-services/user-data-fn";
import { useLiveQuery } from "dexie-react-hooks";
import { motion, AnimatePresence } from "framer-motion";
import { GetOrganizationLocationPinCodeDataFn } from "../../query-mutation-services/organization-data-fn";

const MotionBox = motion(Box);
const UserEditForm = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const [, setmenu] = useAtom(displayMenu);

	const user = useLiveQuery(() => db.user.toArray());

	const userId = useParams({
		from: "/user/edit/$userId",
		select: (params) => params.userId,
	});
	console.log(userId, "User ID");

	useMountEffect(() => {
		setHeaderText("Edit User");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
		setmenu(false);
	});
	const updateMutation = UpdateUserDataFn();
	const filteredData = (user || []).filter(
		(item) => userId && item.id.includes(userId),
	)[0];

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
						initial={{ opacity: 0, x: 50 }}
						animate={{ opacity: 1, x: 0 }}
						exit={{ opacity: 0, x: 50 }}
						transition={{ duration: 0.65 }}
					>
						<UserForm
							onSubmit={(p) => {
								const editVal: UserUpdate = {
									id: userId,
									...p,
								};
								updateMutation.mutateAsync(editVal);
							}}
							lastUpdatedInput={db.getLastUpdated}
							UserId={userId}
							initialValues={filteredData}
							edit={true}
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

export default UserEditForm;
