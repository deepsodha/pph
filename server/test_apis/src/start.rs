#[cfg(test)]
mod tests {
    use log::info;
    use reqwest::StatusCode;
    use std::time::Instant;

    #[test]
    fn test_getting_started() {
        let start = Instant::now();
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
    }

    #[tokio::test]
    pub async fn health_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res = reqwest::get("http://localhost:5000/health-check").await?;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert_eq!(res.status(), StatusCode::OK);

        Ok(())
    }
}
