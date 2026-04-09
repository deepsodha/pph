//import "./globals.css";
import { Inter } from "next/font/google";
import { ChakraProvider } from "@chakra-ui/react";
import Header from "./component/header";
import Footer from "./component/footer";
import theme from "./component/theme";

const inter = Inter({ subsets: ["latin"] });

export default function RootLayout({
	children,
}: {
	children: React.ReactNode;
}): JSX.Element {
	return (
		<html lang="en">
			<body className={inter.className}>
				<ChakraProvider theme={theme}>
					{" "}
					<Header />
					{children}
					<Footer />
				</ChakraProvider>
			</body>
		</html>
	);
}
