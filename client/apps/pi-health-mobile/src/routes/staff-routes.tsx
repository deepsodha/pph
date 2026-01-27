import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import HeaderLayout from "../layout/header-layout";
import { rootRoute } from "./root-route";

export const officestaffRootRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "/staff",
	component: () => <HeaderLayout />,
});
export const officestaffListRoute = createRoute({
	getParentRoute: () => officestaffRootRoute,
	path: "/list",
	component: lazyRouteComponent(() => import("../features/staff/staff-list")),
});
export const officestaffAddRoute = createRoute({
	getParentRoute: () => officestaffRootRoute,
	path: "/add",
	component: lazyRouteComponent(
		() => import("../features/staff/staff-details-add"),
	),
});
export const officestaffEditRoute = createRoute({
	getParentRoute: () => officestaffRootRoute,
	path: "/edit/$staffId",
	component: lazyRouteComponent(
		() => import("../features/staff/staff-details-edit"),
	),
});

export const officestaffRoute = officestaffRootRoute.addChildren([
	officestaffListRoute,
	officestaffAddRoute,
	officestaffEditRoute,
]);
