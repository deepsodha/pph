import { createHashHistory, createRouter } from "@tanstack/react-router";
import { rootRoute } from "./routes/root-route";
import { LoginRoute } from "./routes/login-route";
import { dashboardRoute } from "./routes/dashbboard-routes";
import { systemadminRoute } from "./routes/systemadmin-routes";
import { doctorRoute } from "./routes/doctor-routes";
import { officestaffRoute } from "./routes/staff-routes";
import { SettingsRoute } from "./routes/settings-routes";
import { serviceLocationRoute } from "./routes/service-location-routes";
import { appointmentRoute } from "./routes/appointment-routes";
import { patientRoute } from "./routes/patient-routes";
import { vitalsRoute } from "./routes/vitals-routes";
import { allergyRoute } from "./routes/allergy-routes";
import { problemRoute } from "./routes/problem-routes";
import { medicationRoute } from "./routes/medication-routes";
import { HistoryRoute } from "./routes/history-routes";
import { immunizationRoute } from "./routes/immunization";
import { prescriptionRoute } from "./routes/prescription-routes";
import { ProfileRoute } from "./routes/profile-routes";
import { hospitalRoute } from "./routes/hospital-routes";
import { notesRoute } from "./routes/notes-routes";
import { userRoute } from "./routes/user-routes";

const history = createHashHistory();

const routeTree = rootRoute.addChildren([
	LoginRoute,
	dashboardRoute,
	systemadminRoute,
	doctorRoute,
	officestaffRoute,
	SettingsRoute,
	serviceLocationRoute,
	appointmentRoute,
	patientRoute,
	vitalsRoute,
	allergyRoute,
	problemRoute,
	medicationRoute,
	HistoryRoute,
	immunizationRoute,
	prescriptionRoute,
	ProfileRoute,
	hospitalRoute,
	notesRoute,
	userRoute,
]);

export const router = createRouter({ routeTree, history: history });
