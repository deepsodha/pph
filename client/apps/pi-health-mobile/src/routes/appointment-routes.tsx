import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import HeaderLayout from "../layout/header-layout";
import { rootRoute } from "./root-route";

export const appointmentRootRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "/appointment",
	component: () => <HeaderLayout />,
});
export const appointmentListRoute = createRoute({
	getParentRoute: () => appointmentRootRoute,
	path: "/list",
	component: lazyRouteComponent(
		() => import("../features/appointment/appointment-list"),
	),
});
export const appointmentAddRoute = createRoute({
	getParentRoute: () => appointmentRootRoute,
	path: "/add",
	component: lazyRouteComponent(
		() => import("../features/appointment/appointment-add"),
	),
});
export const appointmentEditRoute = createRoute({
	getParentRoute: () => appointmentRootRoute,
	path: "/edit/$appointmentId",
	component: lazyRouteComponent(
		() => import("../features/appointment/appointment-edit"),
	),
});

export const appointmentRoute = appointmentRootRoute.addChildren([
	appointmentListRoute,
	appointmentAddRoute,
	appointmentEditRoute,
]);
