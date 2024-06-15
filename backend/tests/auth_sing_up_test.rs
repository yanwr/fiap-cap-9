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
    use crate::commons::{AuthCommons, TestContext};

    fn build_request(body: Value) -> Request<Body> {
        Request::builder()
            .method(Method::POST)
            .uri(String::from("/auth/singup"))
            .header("Content-Type", "application/json")
            .body(Body::from(body.to_string()))
            .expect("Failed to build request")
    }

    fn build_request_body(email: &String, password: &String) -> Value {
        json!({
            "email": email,
            "password": password
        })
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    #[serial]
    async fn should_return_sucess_when_sing_up(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let email = String::from("user@fiap.com.br");
        let password = String::from("my$ecr3T");
        let request = build_request(build_request_body(&email, &password));

        let response = ctx.app.clone().oneshot(request).await?;

        assert_eq!(StatusCode::OK, response.status());
        AuthCommons::valid_customer_inserted(&ctx, response).await;
        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    #[serial]
    async fn should_return_error_when_sing_up_with_email_is_empty(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let email = String::from("");
        let password = String::from("my$ecr3T");
        let request = build_request(build_request_body(&email, &password));

        let response = ctx.app.clone().oneshot(request).await?;

        assert_eq!(StatusCode::BAD_REQUEST, response.status());
        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    #[serial]
    async fn should_return_error_when_sing_up_with_password_is_empty(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let email = String::from("user@fiap.com.br");
        let password = String::from("");
        let request = build_request(build_request_body(&email, &password));

        let response = ctx.app.clone().oneshot(request).await?;

        assert_eq!(StatusCode::BAD_REQUEST, response.status());
        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    #[serial]
    async fn should_return_error_when_sing_up_with_email_already_used(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let email = String::from("user@fiap.com.br");
        let password = String::from("my$ecr3T");
        AuthCommons::craete_customer(&ctx, &email, &password).await;
        let request = build_request(build_request_body(&email, &password));

        let response = ctx.app.clone().oneshot(request).await?;

        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, response.status());
        Ok(())
    }
}
