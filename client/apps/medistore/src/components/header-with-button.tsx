import { useAtom } from "jotai";
import { dasboardValue, formValue, profileValue } from "../atoms/header";
import { Link, useRouter } from "@tanstack/react-router";
import { Box, Flex, Heading, Spacer, Image } from "@chakra-ui/react";
import Backarrow from "../assets/Backarrow.svg";
import Logo from "../assets/Logo.svg";
import Bell from "../assets/bell.svg";
const HeaderWithButton = () => {
	const [dashboard] = useAtom(dasboardValue);
	const [profile] = useAtom(profileValue);
	const [form] = useAtom(formValue);
	const router = useRouter();
	return (
		<Box>
			{form && (
				<Box bg="#DDF0EE" top={0} height="100%">
					<Flex align="center" justify="space-between">
						<Box
							pt="4"
							pb="4"
							display="flex"
							alignItems="center"
							justifyContent="center"
						>
							<Link onClick={() => router.history.back()}>
								<Image height={35} width={38} src={Backarrow} />
							</Link>
							<Heading as="h3" size="lg" color="#1A998E" ml="2">
								{"Add Drug"}
							</Heading>
						</Box>
					</Flex>
				</Box>
			)}

			{profile && (
				<Box bg="#DDF0EE" top={0} height="100%">
					<Flex align="center" justify="space-between">
						<Box
							pt="4"
							pb="4"
							display="flex"
							alignItems="center"
							justifyContent="center"
						>
							<Heading as="h3" size="lg" color="#1A998E" ml="2">
								{"My Profile"}
							</Heading>
						</Box>
					</Flex>
				</Box>
			)}
			{dashboard && (
				<Box bg="#DDF0EE" top={0} height="100%">
					<Flex mb="12px" mr="20px" ml="20px">
						<Box display="flex" alignItems="center">
							<Image height="32px" width="82px" src={Logo} />
						</Box>
						<Spacer />
						<Box display="flex" alignItems="center" mb="12px">
							<Image mt={3} height={30} width={34} src={Bell} />
						</Box>
					</Flex>
				</Box>
			)}
		</Box>
	);
};

export default HeaderWithButton;
