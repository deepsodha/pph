import { Route, lazyRouteComponent } from "@tanstack/react-router";
import { rootRoute } from "./root-route";

export const LoginRoute = new Route({
	getParentRoute: () => rootRoute,
	path: "/",
	component: lazyRouteComponent(() => import("../features/auth/login")),
});
