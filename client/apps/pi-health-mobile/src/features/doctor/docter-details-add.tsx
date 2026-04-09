import { Box } from "@chakra-ui/react";
import { AddDoctorDetailDataFn } from "../../query-mutation-services/doctor-detail-data-fn";
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
import { DocterDetailsForm } from "@repo/ui/forms";
import type { DoctorAdd } from "@repo/types/dto";
import { db } from "../../db/db";
import { useLiveQuery } from "dexie-react-hooks";
import { useMountEffect } from "@react-hookz/web";

import { motion, AnimatePresence } from "framer-motion";

const MotionBox = motion(Box);
const DocterDetailsAddForm = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const [, setmenu] = useAtom(displayMenu);
	const addMutation = AddDoctorDetailDataFn();

	const user = useLiveQuery(() => db.user.toArray());

	useMountEffect(() => {
		setHeaderText("Add New Doctor");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
		setmenu(false);
	});
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
						<DocterDetailsForm
							onSubmit={(p) => {
								addMutation.mutateAsync(p as DoctorAdd);
							}}
							user={user}
							image_url1={hideEyeIcon}
							image_url2={EyeIcon}
							edit={false}
							lastUpdatedInput={db.getLastUpdated}
						/>
					</MotionBox>
				</Box>
			</AnimatePresence>
		</>
	);
};

export default DocterDetailsAddForm;
