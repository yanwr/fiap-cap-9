mod commons;

#[cfg(test)]
mod test {
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use rstest::rstest;
    use serial_test::serial;
    use test_context::test_context;
    use tower::ServiceExt;
    use uuid::Uuid;
    use crate::commons::{BiometricsCommons, TestContext};

    fn build_request(url: String) -> Request<Body> {
        Request::builder()
            .method(Method::GET)
            .uri(String::from(format!("/biometrics/actions/get/{}", url)))
            .header("Content-Type", "application/json")
            .body(Body::empty())
            .expect("Failed to build request")
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    #[serial]
    async fn should_return_biometric(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (customer, _) = BiometricsCommons::craete_biometrics(&ctx, &String::from("user@fiap.com.br"), &String::from("pass"), &String::from("s3://image")).await;
        let request = build_request(customer.id.to_string());

        let response = ctx.app.clone().oneshot(request).await?;
        assert_eq!(StatusCode::OK, response.status());
        BiometricsCommons::valid_biometric_inserted(&ctx, response).await;
        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    #[serial]
    async fn should_return_error_when_customer_id_is_empty(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = build_request(Uuid::nil().to_string());

        let response = ctx.app.clone().oneshot(request).await?;
        assert_eq!(StatusCode::BAD_REQUEST, response.status());
        Ok(())
    }
    
    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    #[serial]
    async fn should_return_error_when_customer_id_invalid(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = build_request(Uuid::now_v7().to_string());

        let response = ctx.app.clone().oneshot(request).await?;
        assert_eq!(StatusCode::NOT_FOUND, response.status());
        Ok(())
    }
}
