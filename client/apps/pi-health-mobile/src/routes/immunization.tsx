import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import { rootRoute } from "./root-route";

export const immunizationRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "/immunization",
	component: lazyRouteComponent(() => import("../components/coming-soon")),
});
