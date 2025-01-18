import { createRootRoute } from "@tanstack/react-router";
import { Layout } from "./route-layout";

export const rootRoute = createRootRoute({
	component: Layout,
});
