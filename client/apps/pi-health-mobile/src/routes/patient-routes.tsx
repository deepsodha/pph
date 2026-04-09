import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import HeaderLayout from "../layout/header-layout";
import { rootRoute } from "./root-route";

export const patientRootRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "/patient",
	component: () => <HeaderLayout />,
});
export const patientListRoute = createRoute({
	getParentRoute: () => patientRootRoute,
	path: "/list",
	component: lazyRouteComponent(
		() => import("../features/patients/patient/patient-list"),
	),
});
export const patientAddRoute = createRoute({
	getParentRoute: () => patientRootRoute,
	path: "/add",
	component: lazyRouteComponent(
		() => import("../features/patients/patient/patient-add"),
	),
});

export const patientDetailsRoute = createRoute({
	getParentRoute: () => patientRootRoute,
	path: "/details/$patientId",
	component: lazyRouteComponent(
		() => import("../features/patients/patient/patient-details"),
	),
});
export const patientEditRoute = createRoute({
	getParentRoute: () => patientRootRoute,
	path: "/edit/$patientId",
	component: lazyRouteComponent(
		() => import("../features/patients/patient/patient-edit"),
	),
});

export const patientRoute = patientRootRoute.addChildren([
	patientListRoute,
	patientAddRoute,
	patientEditRoute,
	patientDetailsRoute,
]);
