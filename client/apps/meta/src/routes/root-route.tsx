import { createRootRoute } from "@tanstack/react-router";
import Layout from "../routes/route-layout";

export const rootRoute = createRootRoute({
	component: Layout,
});
