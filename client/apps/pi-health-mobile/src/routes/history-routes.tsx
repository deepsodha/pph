import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import { rootRoute } from "./root-route";

export const HistoryRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "/history",
	component: lazyRouteComponent(() => import("../components/coming-soon")),
});
