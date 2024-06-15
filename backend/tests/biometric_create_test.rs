mod commons;

#[cfg(test)]
mod test {
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use rstest::rstest;
    use serde_json::{json, Value};
    use serial_test::serial;
    use test_context::test_context;
    use tower::ServiceExt;
    use uuid::Uuid;
    use crate::commons::{AuthCommons, BiometricsCommons, TestContext};

    fn build_request(body: Value) -> Request<Body> {
        Request::builder()
            .method(Method::POST)
            .uri(String::from("/biometrics/actions/create"))
            .header("Content-Type", "application/json")
            .body(Body::from(body.to_string()))
            .expect("Failed to build request")
    }

    fn build_request_body(customer_id: &Uuid, image_path: &String) -> Value {
        json!({
            "customer_id": customer_id,
            "image_path": image_path
        })
    }
    fn build_request_body_customer_id_empty(image_path: &String) -> Value {
        json!({
            "customer_id": "",
            "image_path": image_path
        })
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    #[serial]
    async fn should_return_sucess_when_create_biometric(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let customer = AuthCommons::craete_customer(&ctx, &String::from("user@fiap.com.br"), &String::from("pass")).await;
        let request = build_request(build_request_body(&customer.id, &String::from("s3//image")));

        let response = ctx.app.clone().oneshot(request).await?;
        assert_eq!(StatusCode::OK, response.status());
        BiometricsCommons::valid_biometric_inserted(&ctx, response).await;
        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    #[serial]
    async fn should_return_error_when_create_biometric_with_customer_id_is_empty(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = build_request(build_request_body_customer_id_empty(&String::from("s3//image")));

        let response = ctx.app.clone().oneshot(request).await?;
        assert_eq!(StatusCode::UNPROCESSABLE_ENTITY, response.status());
        Ok(())
    }
    
    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    #[serial]
    async fn should_return_error_when_create_biometric_with_image_path_is_empty(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let customer = AuthCommons::craete_customer(&ctx, &String::from("user@fiap.com.br"), &String::from("pass")).await;
        let request = build_request(build_request_body(&customer.id, &String::from("")));

        let response = ctx.app.clone().oneshot(request).await?;
        assert_eq!(StatusCode::BAD_REQUEST, response.status());
        Ok(())
    }
    
    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    #[serial]
    async fn should_return_error_when_create_biometric_with_customer_id_invalid(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = build_request(build_request_body(&Uuid::now_v7(), &String::from("s3://image")));

        let response = ctx.app.clone().oneshot(request).await?;
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, response.status());
        Ok(())
    }
}
