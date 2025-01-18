import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import HeaderLayout from "../layout/header-layout";
import { rootRoute } from "./root-route";

export const userRootRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "/user",
	component: () => <HeaderLayout />,
});
export const userListRoute = createRoute({
	getParentRoute: () => userRootRoute,
	path: "/list",
	component: lazyRouteComponent(() => import("../features/user/user-list")),
});
export const userAddRoute = createRoute({
	getParentRoute: () => userRootRoute,
	path: "/add",
	component: lazyRouteComponent(() => import("../features/user/user-add")),
});
export const userEditRoute = createRoute({
	getParentRoute: () => userRootRoute,
	path: "/edit/$userId",
	component: lazyRouteComponent(() => import("../features/user/user-edit")),
});

export const userRoute = userRootRoute.addChildren([
	userListRoute,
	userAddRoute,
	userEditRoute,
]);
