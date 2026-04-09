import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import { rootRoute } from "./root-route";
import HeaderLayout from "../layout/header-layout";

const drugRootRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "/drug",
	component: () => <HeaderLayout />,
});
const drugListRoute = createRoute({
	getParentRoute: () => drugRootRoute,
	path: "/list",
	component: lazyRouteComponent(() => import("../drug/drug-list")),
});
const drugAddRoute = createRoute({
	getParentRoute: () => drugRootRoute,
	path: "/add",
	component: lazyRouteComponent(() => import("../drug/drug-add")),
});
const drugEditRoute = createRoute({
	getParentRoute: () => drugRootRoute,
	path: "/edit/$drugId",
	component: lazyRouteComponent(() => import("../drug/drug-edit")),
});

export const drugRoute = drugRootRoute.addChildren([
	drugListRoute,
	drugAddRoute,
	drugEditRoute,
]);
