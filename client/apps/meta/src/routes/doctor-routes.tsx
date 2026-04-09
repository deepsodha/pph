import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import { rootRoute } from "./root-route";
import HeaderLayout from "../layout/header-layout";

const doctorRootRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "/doctor",
	component: () => <HeaderLayout />,
});
const doctorListRoute = createRoute({
	getParentRoute: () => doctorRootRoute,
	path: "/list",
	component: lazyRouteComponent(() => import("../doctor/doctor-list")),
});
const doctorAddRoute = createRoute({
	getParentRoute: () => doctorRootRoute,
	path: "/add",
	component: lazyRouteComponent(() => import("../doctor/doctor-add")),
});
const doctorEditRoute = createRoute({
	getParentRoute: () => doctorRootRoute,
	path: "/edit/$doctorId",
	component: lazyRouteComponent(() => import("../doctor/doctor-edit")),
});

export const doctorRoute = doctorRootRoute.addChildren([
	doctorListRoute,
	doctorAddRoute,
	doctorEditRoute,
]);
