//import "./globals.css";

import { Inter } from "next/font/google";
import { ChakraProvider } from "@chakra-ui/react";
const inter = Inter({ subsets: ["latin"] });

export default function RootLayout({
	children,
}: {
	children: React.ReactNode;
}): JSX.Element {
	return (
		<html lang="en">
			<body className={inter.className}>
				<ChakraProvider>{children}</ChakraProvider>
			</body>
		</html>
	);
}
