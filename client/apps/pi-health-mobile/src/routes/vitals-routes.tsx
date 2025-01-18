import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import HeaderLayout from "../layout/header-layout";
import { rootRoute } from "./root-route";

export const vitalsRootRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "/vitals",
	component: () => <HeaderLayout />,
});
export const vitalsListRoute = createRoute({
	getParentRoute: () => vitalsRootRoute,
	path: "/list/$patientId",
	component: lazyRouteComponent(
		() => import("../features/patients/vitals/vitals-dashboard"),
	),
});
export const vitalsAddRoute = createRoute({
	getParentRoute: () => vitalsRootRoute,
	path: "/add/$patientId",
	component: lazyRouteComponent(
		() => import("../features/patients/vitals/vitals-add"),
	),
});
export const vitalsEditRoute = createRoute({
	getParentRoute: () => vitalsRootRoute,
	path: "/edit/$patientId/$VitalsId",
	component: lazyRouteComponent(
		() => import("../features/patients/vitals/vitals-edit"),
	),
});

export const vitalsRoute = vitalsRootRoute.addChildren([
	vitalsListRoute,
	vitalsAddRoute,
	vitalsEditRoute,
]);
