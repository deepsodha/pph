import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import HeaderLayout from "../layout/header-layout";
import { rootRoute } from "./root-route";

export const notesRootRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "/notes",
	component: () => <HeaderLayout />,
});
export const notesListRoute = createRoute({
	getParentRoute: () => notesRootRoute,
	path: "/list/$patientId",
	component: lazyRouteComponent(
		() => import("../features/patients/notes/notes-list"),
	),
});
export const notesAddRoute = createRoute({
	getParentRoute: () => notesRootRoute,
	path: "/add/$patientId",
	component: lazyRouteComponent(
		() => import("../features/patients/notes/notes"),
	),
});
export const notesEditRoute = createRoute({
	getParentRoute: () => notesRootRoute,
	path: "/edit/$patientId/$notesId",
	component: lazyRouteComponent(
		() => import("../features/patients/notes/notes"),
	),
});

export const notesRoute = notesRootRoute.addChildren([
	notesListRoute,
	notesAddRoute,
	notesEditRoute,
]);
