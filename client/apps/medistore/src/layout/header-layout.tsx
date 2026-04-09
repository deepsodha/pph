import { Outlet } from "@tanstack/react-router";
import HeaderWithButton from "../components/header-with-button";

const HeaderLayout = () => {
	return (
		<div>
			<HeaderWithButton />
			<Outlet />
		</div>
	);
};

export default HeaderLayout;
