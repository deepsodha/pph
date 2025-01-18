import {
	Box,
	Modal,
	ModalCloseButton,
	ModalContent,
	ModalHeader,
	ModalOverlay,
} from "@chakra-ui/react";
import TextField from "../../components/text-field";
import { GetOrganizationByIdDataFn } from "../../query-mutation-services/organization-data-fn";

type OrganizationPopupProps = {
	open: boolean;
	close: () => void;
	id: string;
};
const OrganizationPopup = ({ open, close, id }: OrganizationPopupProps) => {
	const { data, isFetched } = GetOrganizationByIdDataFn(id);
	const shouldOpen = open && isFetched;

	return (
		<div>
			<Modal isOpen={shouldOpen} onClose={close} motionPreset={"slideInBottom"}>
				<ModalOverlay />
				<ModalContent mr={5} ml={5}>
					<ModalHeader textAlign="center">Organization Details</ModalHeader>
					<ModalCloseButton />
					{data?.result?.map((items) => (
						<Box margin="1%" key={items.id}>
							<TextField heading={"Name:"} result={items.name} />
							<TextField heading={"Details:"} result={items.details} />
							<TextField
								heading={"Phone Number:"}
								result={items.mobile_number}
							/>
							<TextField heading={"Email:"} result={items.email} />
							<TextField heading={"Address:"} result={items.address_line} />
							<TextField heading={"Pin Code:"} result={items.pin_code} />
							<TextField heading={"City:"} result={items.city} />

							<TextField heading={"State:"} result={items.state} />
							<TextField heading={"Country:"} result={items.country} />
						</Box>
					))}
				</ModalContent>
			</Modal>
		</div>
	);
};

export default OrganizationPopup;
