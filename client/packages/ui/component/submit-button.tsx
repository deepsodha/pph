import { Button, Center } from "@chakra-ui/react";
type SubmitButtonProps = {
	loading: boolean;
};
export const SubmitButton = ({ loading }: SubmitButtonProps): JSX.Element => {
	return (
		<div style={{ bottom: "0", left: "50%" }}>
			<Center>
				<Button
					size="md"
					type="submit"
					height="48px"
					width="375px"
					color={"#FFFFFF"}
					bgColor={"#095FBA"}
					border="2px"
					mt={8}
					isLoading={loading}
					loadingText="Submitting"
				>
					Submit
				</Button>
			</Center>
		</div>
	);
};
