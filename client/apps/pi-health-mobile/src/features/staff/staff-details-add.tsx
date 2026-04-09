import { Box } from "@chakra-ui/react";
import { useAtom } from "jotai";
import EyeIcon from "../../assets/confirm_password_icon.svg";
import hideEyeIcon from "../../assets/hideyeicon.svg";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
	displayMenu,
} from "../../atoms/header";
import { StaffDetailsForm } from "@repo/ui/forms";
import type { StaffAdd } from "@repo/types/dto";
import { db } from "../../db/db";
import { useLiveQuery } from "dexie-react-hooks";
import { useMountEffect } from "@react-hookz/web";
import { AddStaffDetailDataFn } from "../../query-mutation-services/staff-detail-data-fn";
import { motion, AnimatePresence } from "framer-motion";

const MotionBox = motion(Box);

const StaffDetailsAdd = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setmenu] = useAtom(displayMenu);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const addMutation = AddStaffDetailDataFn();

	useMountEffect(() => {
		setHeaderText("Add Staff");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
		setmenu(false);
	});

	const user = useLiveQuery(() => db.user.toArray());

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
						<StaffDetailsForm
							onSubmit={(p) => {
								addMutation.mutateAsync(p as StaffAdd);
							}}
							image_url1={hideEyeIcon}
							image_url2={EyeIcon}
							edit={false}
							user={user}
							lastUpdatedInput={db.getLastUpdated}
						/>
					</MotionBox>
				</Box>
			</AnimatePresence>
		</>
	);
};

export default StaffDetailsAdd;
