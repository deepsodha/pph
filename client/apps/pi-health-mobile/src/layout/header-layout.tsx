import { Outlet } from "@tanstack/react-router";
import HeaderWithButton from "../components/header-with-button";
import { useEffect } from "react";
import { db } from "../db/db";

const HeaderLayout = () => {
	//put resert ones or stopsync on logout
	useEffect(() => {
		db.startSync(60);
	}, []);
	return (
		<div>
			<HeaderWithButton />
			<Outlet />
		</div>
	);
};

export default HeaderLayout;
