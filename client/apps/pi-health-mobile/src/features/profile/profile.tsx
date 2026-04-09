import LogoutButton from "../../components/logout-button";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
	profileValue,
	displayMenu,
} from "../../atoms/header";
import { useAtom } from "jotai";
import { useMountEffect } from "@react-hookz/web";
import { Box, Image } from "@chakra-ui/react";
import { Link } from "@tanstack/react-router";
import { router } from "../../App";
import Backarrow from "../../assets/Back Arrow Icon.svg";
const profile = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const [, setProfileValue] = useAtom(profileValue);
	const [, setmenu] = useAtom(displayMenu);
	useMountEffect(() => {
		setHeaderText("Pi Health");
		setAddValue(true);
		setFormValue(false);
		setDashboardValue(false);
		setProfileValue(false);
		setmenu(false);
	});
	return (
		<div>
			<Link onClick={() => router.history.back()}>
				<Image height={35} width={38} src={Backarrow} />
			</Link>
			<Box padding="6%">
				<LogoutButton />
			</Box>
		</div>
	);
};
export default profile;
