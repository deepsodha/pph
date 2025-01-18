import { Button, Image } from "@chakra-ui/react";
import Edit from "../assets/Edit Icon.svg";
import { Link } from "@tanstack/react-router";

type EditButtonProps = {
	path?: string;
};

const EditButton = ({ path }: EditButtonProps) => {
	return (
		<div>
			<Link to={path}>
				<Button ml={1} bg={"none"}>
					<Image height="75%" width="75%" src={Edit} />
				</Button>
			</Link>
		</div>
	);
};

export default EditButton;
