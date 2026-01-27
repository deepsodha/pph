import { Outlet } from "@tanstack/react-router";
import Header from "../components/header";

const HeaderLayout = () => {
	return (
		<div>
			<Header />
			<Outlet />
		</div>
	);
};

export default HeaderLayout;
