import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import HeaderLayout from "../layout/header-layout";
import { rootRoute } from "./root-route";

export const prescriptionRootRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "/prescription",
	component: () => <HeaderLayout />,
});
export const prescriptionListRoute = createRoute({
	getParentRoute: () => prescriptionRootRoute,
	path: "/list",
	component: lazyRouteComponent(
		() => import("../features/prescription/prescription-list"),
	),
});
export const prescriptionAddRoute = createRoute({
	getParentRoute: () => prescriptionRootRoute,
	path: "/add",
	component: lazyRouteComponent(
		() => import("../features/prescription/prescription-add"),
	),
});
export const prescriptionEditRoute = createRoute({
	getParentRoute: () => prescriptionRootRoute,
	path: "/edit/$prescriptionId",
	component: lazyRouteComponent(
		() => import("../features/prescription/prescription-edit"),
	),
});

export const prescriptionRoute = prescriptionRootRoute.addChildren([
	prescriptionListRoute,
	prescriptionAddRoute,
	prescriptionEditRoute,
]);
