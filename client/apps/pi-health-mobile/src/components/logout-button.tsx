import { Button, Image, useToast } from "@chakra-ui/react";
import Logoutimg from "../../src/assets/Logout icon.svg";
import { db } from "../db/db";
import { useNavigate } from "@tanstack/react-router";
import { useAtom } from "jotai";
import {
	issuperadmin,
	issystemadmin,
	isdoctor,
	isofficestaff,
} from "../atoms/roles";

const LogoutButton = () => {
	const [superadmin, setsuperadmin] = useAtom(issuperadmin);
	const [systemadmin, setsystemadmin] = useAtom(issystemadmin);
	const [doctor, setdoctor] = useAtom(isdoctor);
	const [officestaff, setofficestaff] = useAtom(isofficestaff);
	const navigate = useNavigate();
	const toast = useToast();
	const handleLogout = () => {
		try {
			db.stopSync();
			localStorage.removeItem("token");
			setsuperadmin(false);
			setsystemadmin(false);
			setdoctor(false);
			setofficestaff(false);
			navigate({ to: "/" });
			console.log(superadmin, "superadmin");
			console.log(systemadmin, "systemadmin");
			console.log(doctor, "doctor");
			console.log(officestaff, "staff");
		} catch (error) {
			toast({
				title: "Error",
				description: error as string,
				status: "error",
				duration: 9000,
				isClosable: true,
			});
		}
	};
	return (
		<div>
			<Button onClick={() => handleLogout()} bgColor={"#FFFFFF"}>
				<Image src={Logoutimg} alt="log out" height={8} width={5} />
			</Button>
		</div>
	);
};

export default LogoutButton;
