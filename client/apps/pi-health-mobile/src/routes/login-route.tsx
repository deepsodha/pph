import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import { rootRoute } from "./root-route";

export const LoginRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "/",
	component: lazyRouteComponent(() => import("../features/auth/login")),
});
