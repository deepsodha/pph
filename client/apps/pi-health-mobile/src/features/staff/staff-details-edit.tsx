import { Box } from "@chakra-ui/react";
import { useParams } from "@tanstack/react-router";
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
import type { StaffAdd, StaffUpdate } from "@repo/types/dto";
import { db } from "../../db/db";
import { useLiveQuery } from "dexie-react-hooks";
import { useMountEffect } from "@react-hookz/web";
import { UpdateStaffDetailsDataFn } from "../../query-mutation-services/staff-detail-data-fn";

import { motion, AnimatePresence } from "framer-motion";

const MotionBox = motion(Box);
const StaffDetailsEditForm = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setmenu] = useAtom(displayMenu);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const updateMutation = UpdateStaffDetailsDataFn();

	const staff = useLiveQuery(() => db.staff.toArray());

	const staffId = useParams({
		from: "/staff/edit/$staffId",
		select: (params) => params.staffId,
	});
	const filteredData = staff?.find((item) => item.id === staffId);

	useMountEffect(() => {
		setHeaderText("Edit Staff");
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
						initial={{ opacity: 0, x: 50 }}
						animate={{ opacity: 1, x: 0 }}
						exit={{ opacity: 0, x: 50 }}
						transition={{ duration: 0.65 }}
					>
						<StaffDetailsForm
							onSubmit={(p) => {
								const editVal: StaffUpdate = {
									id: staffId,
									input: { ...(p as StaffAdd) },
								};
								updateMutation.mutateAsync(editVal);
							}}
							image_url1={hideEyeIcon}
							image_url2={EyeIcon}
							StaffId={staffId}
							initialValues={filteredData}
							edit={true}
							user={user}
							lastUpdatedInput={db.getLastUpdated}
						/>
					</MotionBox>
				</Box>
			</AnimatePresence>
		</>
	);
};

export default StaffDetailsEditForm;
