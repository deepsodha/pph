import { Button, Image } from "@chakra-ui/react";
import Delete from "../assets/Delete Icon.svg";
type DeleteButtonProps = {
	onclick?: () => void;
};
const DeleteButton = ({ onclick }: DeleteButtonProps) => {
	return (
		<div>
			<Button bg={"none"} ml={1} onClick={onclick}>
				<Image height="75%" width="75%" src={Delete} />
			</Button>
		</div>
	);
};

export default DeleteButton;
