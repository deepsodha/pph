"use client";
import { ChakraProvider } from "@chakra-ui/react";
import theme from "./components/theme";
export function Providers({
	children,
}: { children: React.ReactNode }): JSX.Element {
	return <ChakraProvider theme={theme}>{children}</ChakraProvider>;
}
