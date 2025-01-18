import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import { rootRoute } from "./root-route";
import HeaderLayout from "../layout/header-layout";

export const hospitalRootRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "hospital",
	component: () => <HeaderLayout />,
});
export const hospitalPageRoute = createRoute({
	getParentRoute: () => hospitalRootRoute,
	path: "/list",
	component: lazyRouteComponent(() => import("../features/hospital/hospital")),
});

export const hospitalRoute = hospitalRootRoute.addChildren([hospitalPageRoute]);
