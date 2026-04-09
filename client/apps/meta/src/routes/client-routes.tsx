import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import { rootRoute } from "./root-route";
import HeaderLayout from "../layout/header-layout";

const clientRootRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "/client",
	component: () => <HeaderLayout />,
});
const clientListRoute = createRoute({
	getParentRoute: () => clientRootRoute,
	path: "/list",
	component: lazyRouteComponent(() => import("../client/client-list")),
});
const clientAddRoute = createRoute({
	getParentRoute: () => clientRootRoute,
	path: "/add",
	component: lazyRouteComponent(() => import("../client/client-add-form")),
});
const clientEditRoute = createRoute({
	getParentRoute: () => clientRootRoute,
	path: "/edit/$clientId",
	component: lazyRouteComponent(() => import("../client/client-edit")),
});

export const clientRoute = clientRootRoute.addChildren([
	clientListRoute,
	clientAddRoute,
	clientEditRoute,
]);
