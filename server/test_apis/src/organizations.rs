#[cfg(test)]
mod tests {
    use common::dto::address::AddressInput;
    use log::info;
    use organization::dto::{
        organization_add::OrganizationAdd, organization_delete::OrganizationDelete,
        organization_update::OrganizationUpdate,
    };
    use services::method::methods::gp_methods::{
        app_methods::AppMethods, organizations::organization::OrganizationMethods,
    };
    use std::time::Instant;

    use crate::helper::{services_helper, URL};

    #[tokio::test]
    pub async fn add_organizations_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Organizations(OrganizationMethods::Add(
                OrganizationAdd {
                    name: String::from("1"),
                    details: String::from("1"),
                    phone_number: String::from("1"),
                    email: String::from("1"),
                    address: AddressInput {
                        pin_code: String::from("1"),
                        city: String::from("1"),
                        state: String::from("1"),
                        address_line: String::from("1"),
                        country: String::from("1"),
                    },
                },
                None,
            )),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn update_organizations_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Organizations(OrganizationMethods::Update(
                OrganizationUpdate {
                    id: String::from("1"),
                    name: String::from("1"),
                    details: String::from("1"),
                    phone_number: String::from("1234567890"),
                    email: String::from("1"),
                    address: AddressInput {
                        pin_code: String::from("1"),
                        city: String::from("1"),
                        state: String::from("1"),
                        address_line: String::from("1"),
                        country: String::from("1"),
                    },
                },
                None,
            )),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn delete_organizations_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Organizations(OrganizationMethods::Delete(
                OrganizationDelete {
                    id: String::from("1"),
                },
                None,
            )),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }
}
