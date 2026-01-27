import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import HeaderLayout from "../layout/header-layout";
import { rootRoute } from "./root-route";

export const systemadminRootRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "/systemadmin",
	component: () => <HeaderLayout />,
});
export const systemadminListRoute = createRoute({
	getParentRoute: () => systemadminRootRoute,
	path: "/list",
	component: lazyRouteComponent(
		() => import("../features/system-admin/system-admin-list"),
	),
});
export const systemadminAddRoute = createRoute({
	getParentRoute: () => systemadminRootRoute,
	path: "/add",
	component: lazyRouteComponent(
		() => import("../features/system-admin/system-admin-add"),
	),
});
export const systemadminEditRoute = createRoute({
	getParentRoute: () => systemadminRootRoute,
	path: "/edit/$systemAdminId",
	component: lazyRouteComponent(
		() => import("../features/system-admin/system-admin-edit"),
	),
});

export const systemadminRoute = systemadminRootRoute.addChildren([
	systemadminListRoute,
	systemadminAddRoute,
	systemadminEditRoute,
]);
