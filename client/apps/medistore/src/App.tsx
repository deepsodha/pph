import { LoginRoute } from "./routes/login-route";
import { rootRoute } from "./routes/root-route";
import { Router } from "@tanstack/react-router";

const routeTree = rootRoute.addChildren([LoginRoute]);

export const router = new Router({ routeTree });
