import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import HeaderLayout from "../layout/header-layout";
import { rootRoute } from "./root-route";

export const problemRootRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "/problem",
	component: () => <HeaderLayout />,
});
export const problemListRoute = createRoute({
	getParentRoute: () => problemRootRoute,
	path: "/list/$patientId",
	component: lazyRouteComponent(
		() => import("../features/patients/problems/problem-list"),
	),
});
export const problemAddRoute = createRoute({
	getParentRoute: () => problemRootRoute,
	path: "/add/$patientId",
	component: lazyRouteComponent(
		() => import("../features/patients/problems/problem-add"),
	),
});
export const problemEditRoute = createRoute({
	getParentRoute: () => problemRootRoute,
	path: "/edit/$patientId/$problemId",
	component: lazyRouteComponent(
		() => import("../features/patients/problems/problem-edit"),
	),
});

export const problemRoute = problemRootRoute.addChildren([
	problemListRoute,
	problemAddRoute,
	problemEditRoute,
]);
