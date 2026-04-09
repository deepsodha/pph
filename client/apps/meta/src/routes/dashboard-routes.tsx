import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import { rootRoute } from "./root-route";
import HeaderLayout from "../layout/header-layout";

export const dashboardRootRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "/dashboard",
	component: () => <HeaderLayout />,
});
export const dashboardListRoute = createRoute({
	getParentRoute: () => dashboardRootRoute,
	path: "/",
	component: lazyRouteComponent(() => import("../dashboard/dashboard")),
});

export const dashboardRoute = dashboardRootRoute.addChildren([
	dashboardListRoute,
]);
