import { Box, Center } from "@chakra-ui/react";
import { useAtom } from "jotai";
import { useMountEffect } from "@react-hookz/web";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
	displayMenu,
} from "../../../atoms/header";
import { motion, AnimatePresence } from "framer-motion";

const MotionBox = motion(Box);

import ServiceLocationCard from "../../../components/service-location-card";
const Settings = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setmenu] = useAtom(displayMenu);
	const [, setDashboardValue] = useAtom(dashboardValue);
	useMountEffect(() => {
		setHeaderText("Settings");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
		setmenu(false);
	});
	return (
		<div>
			<AnimatePresence>
				<Box bgColor={"#F7F7F9"} minHeight={"100vh"}>
					<Center>
						<MotionBox
							initial={{ opacity: 0, x: -50 }}
							animate={{ opacity: 1, x: 0 }}
							exit={{ opacity: 0, x: -50 }}
							transition={{ duration: 0.65 }}
							width={{ md: "90%", base: "90%", lg: "70%" }}
						>
							<ServiceLocationCard path={"/serviceLocation/list"} />
						</MotionBox>
					</Center>
				</Box>
			</AnimatePresence>
		</div>
	);
};

export default Settings;
