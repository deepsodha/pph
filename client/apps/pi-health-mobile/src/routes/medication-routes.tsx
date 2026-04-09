import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import HeaderLayout from "../layout/header-layout";
import { rootRoute } from "./root-route";

export const medicationRootRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "/medication",
	component: () => <HeaderLayout />,
});
export const medicationListRoute = createRoute({
	getParentRoute: () => medicationRootRoute,
	path: "/list/$patientId",
	component: lazyRouteComponent(
		() => import("../features/patients/medication/medication-list"),
	),
});
export const medicationAddRoute = createRoute({
	getParentRoute: () => medicationRootRoute,
	path: "/add/$patientId",
	component: lazyRouteComponent(
		() => import("../features/patients/medication/medication-add"),
	),
});
export const medicationEditRoute = createRoute({
	getParentRoute: () => medicationRootRoute,
	path: "/edit/$patientId/$medicationId",
	component: lazyRouteComponent(
		() => import("../features/patients/medication/medication-edit"),
	),
});

export const medicationRoute = medicationRootRoute.addChildren([
	medicationListRoute,
	medicationAddRoute,
	medicationEditRoute,
]);
