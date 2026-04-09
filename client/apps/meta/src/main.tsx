import React from "react";
import ReactDOM from "react-dom/client";
import { router } from "./App.tsx";
import { ChakraProvider } from "@chakra-ui/react";
import { RouterProvider } from "@tanstack/react-router";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

const queryClient = new QueryClient({});
const rootElement = document.getElementById("root");
if (rootElement && !rootElement.innerHTML) {
	const root = ReactDOM.createRoot(rootElement);

	root.render(
		<React.StrictMode>
			<ChakraProvider>
				<QueryClientProvider client={queryClient}>
					<RouterProvider router={router} />
				</QueryClientProvider>
			</ChakraProvider>
		</React.StrictMode>,
	);
}
