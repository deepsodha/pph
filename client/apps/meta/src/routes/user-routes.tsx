import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import { rootRoute } from "./root-route";
import HeaderLayout from "../layout/header-layout";

const userRootRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "/user",
	component: () => <HeaderLayout />,
});
const userListRoute = createRoute({
	getParentRoute: () => userRootRoute,
	path: "/list",
	component: lazyRouteComponent(() => import("../user/user-list")),
});
const userAddRoute = createRoute({
	getParentRoute: () => userRootRoute,
	path: "/add",
	component: lazyRouteComponent(() => import("../user/user-add")),
});
const userEditRoute = createRoute({
	getParentRoute: () => userRootRoute,
	path: "/edit/$userId",
	component: lazyRouteComponent(() => import("../user/user-edit")),
});

export const userRoute = userRootRoute.addChildren([
	userListRoute,
	userAddRoute,
	userEditRoute,
]);
