import { createHashHistory, createRouter } from "@tanstack/react-router";
import { rootRoute } from "./routes/root-route";
import { drugRoute } from "./routes/drug-routes";
import { doctorRoute } from "./routes/doctor-routes";
import { clientRoute } from "./routes/client-routes";
import { userRoute } from "./routes/user-routes";
import { dashboardRoute } from "./routes/dashboard-routes";
import { loginRoute } from "./routes/login-routs";

const history = createHashHistory();

const routeTree = rootRoute.addChildren([
	loginRoute,
	dashboardRoute,
	drugRoute,
	doctorRoute,
	clientRoute,
	userRoute,
]);

export const router = createRouter({ routeTree, history: history });
