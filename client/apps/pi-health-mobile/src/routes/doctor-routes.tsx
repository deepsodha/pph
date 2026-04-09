import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import HeaderLayout from "../layout/header-layout";
import { rootRoute } from "./root-route";

export const doctorRootRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "/doctor",
	component: () => <HeaderLayout />,
});
export const doctorListRoute = createRoute({
	getParentRoute: () => doctorRootRoute,
	path: "/list",
	component: lazyRouteComponent(() => import("../features/doctor/docter-list")),
});
export const doctorAddRoute = createRoute({
	getParentRoute: () => doctorRootRoute,
	path: "/add",
	component: lazyRouteComponent(
		() => import("../features/doctor/docter-details-add"),
	),
});
export const doctorEditRoute = createRoute({
	getParentRoute: () => doctorRootRoute,
	path: "/edit/$doctorId",
	component: lazyRouteComponent(
		() => import("../features/doctor/docter-details-edit"),
	),
});

export const doctorRoute = doctorRootRoute.addChildren([
	doctorListRoute,
	doctorAddRoute,
	doctorEditRoute,
]);
