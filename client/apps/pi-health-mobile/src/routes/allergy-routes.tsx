import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import HeaderLayout from "../layout/header-layout";
import { rootRoute } from "./root-route";

export const allergyRootRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "/allergy",
	component: () => <HeaderLayout />,
});
export const allergyListRoute = createRoute({
	getParentRoute: () => allergyRootRoute,
	path: "/list/$patientId",
	component: lazyRouteComponent(
		() => import("../features/patients/allergy/allergy-list"),
	),
});
export const allergyAddRoute = createRoute({
	getParentRoute: () => allergyRootRoute,
	path: "/add/$patientId",
	component: lazyRouteComponent(
		() => import("../features/patients/allergy/alergy-add"),
	),
});
export const allergyEditRoute = createRoute({
	getParentRoute: () => allergyRootRoute,
	path: "/edit/$patientId/$allergyId",
	component: lazyRouteComponent(
		() => import("../features/patients/allergy/allergy-edit"),
	),
});

export const allergyRoute = allergyRootRoute.addChildren([
	allergyListRoute,
	allergyAddRoute,
	allergyEditRoute,
]);
