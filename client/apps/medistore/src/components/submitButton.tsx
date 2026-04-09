import { Button, Center } from "@chakra-ui/react";
export const SubmitButton = () => {
	return (
		<div style={{ bottom: "0", left: "50%" }}>
			<Center>
				<Button
					size="md"
					type="submit"
					height="48px"
					width="100%"
					color={"#FFFFFF"}
					bgColor={"#1A998E"}
					border="2px"
					bottom={10}
					loadingText="Submitting"
				>
					Submit
				</Button>
			</Center>
		</div>
	);
};
