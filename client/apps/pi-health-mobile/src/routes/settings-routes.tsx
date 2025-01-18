import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import { rootRoute } from "./root-route";
import HeaderLayout from "../layout/header-layout";

export const SettingsRootRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "/settings",
	component: () => <HeaderLayout />,
});
export const SettingsListRoute = createRoute({
	getParentRoute: () => SettingsRootRoute,
	path: "/list",
	component: lazyRouteComponent(
		() => import("../features/patients/settings/settings"),
	),
});

export const SettingsRoute = SettingsRootRoute.addChildren([SettingsListRoute]);
