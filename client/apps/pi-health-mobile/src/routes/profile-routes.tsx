import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import { rootRoute } from "./root-route";

export const ProfileRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "/profile",
	component: lazyRouteComponent(() => import("../features/profile/profile")),
});
