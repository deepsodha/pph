import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import { rootRoute } from "./root-route";
import HeaderLayout from "../layout/header-layout";

export const serviceLocationRootRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "/servicelocation",
	component: () => <HeaderLayout />,
});
export const serviceLocationListRoute = createRoute({
	getParentRoute: () => serviceLocationRootRoute,
	path: "/list",
	component: lazyRouteComponent(
		() => import("../features/service-location/service-location-details-list"),
	),
});

export const serviceLocationAddRoute = createRoute({
	getParentRoute: () => serviceLocationRootRoute,
	path: "/add",
	component: lazyRouteComponent(
		() => import("../features/service-location/service-location-details-add"),
	),
});

export const serviceLocationEditRoute = createRoute({
	getParentRoute: () => serviceLocationRootRoute,
	path: "/edit/$servicelocationId",
	component: lazyRouteComponent(
		() => import("../features/service-location/service-location-details-edit"),
	),
});

export const serviceLocationRoute = serviceLocationRootRoute.addChildren([
	serviceLocationListRoute,
	serviceLocationAddRoute,
	serviceLocationEditRoute,
]);
