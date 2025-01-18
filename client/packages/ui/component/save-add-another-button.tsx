import { Button, Center } from "@chakra-ui/react";
type SaveAddAnotherButtonProps = {
	path: string;
};

export const SaveAddAnotherButton = (props: SaveAddAnotherButtonProps) => {
	console.log(props.path);

	return (
		<div>
			<Center>
				<Button
					hidden={true} //will be displayed when needed
					size="md"
					height="48px"
					mt={5}
					type="submit"
					width="370px"
					color={"#095FBA"}
					bgColor={"#FFFFFF"}
					border="2px"
					marginBottom="20px"
				>
					Save & Add Another
				</Button>
			</Center>
		</div>
	);
};
