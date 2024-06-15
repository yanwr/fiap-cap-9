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
    use crate::commons::{body_as_json_value, AuthCommons, TestContext};

    fn build_request(body: Value) -> Request<Body> {
        Request::builder()
            .method(Method::POST)
            .uri(String::from("/auth/singin"))
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
    async fn should_return_sucess_when_sing_in(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let email = String::from("user@fiap.com.br");
        let password = String::from("my$ecr3T");
        AuthCommons::craete_customer(&ctx, &email, &password).await;
        let request = build_request(build_request_body(&email, &password));

        let response = ctx.app.clone().oneshot(request).await?;
        let (parts, body) = response.into_parts();
        let body_json = body_as_json_value(body).await;
        assert_eq!(StatusCode::OK, parts.status);
        assert!(parts.headers.get("Authorization").is_some());
        assert_eq!(email, body_json.get("email").expect("Failed to read email").as_str().expect("failed to parse str"));
        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    #[serial]
    async fn should_return_error_when_sing_in_with_email_is_empty(
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
    async fn should_return_error_when_sing_in_with_password_is_empty(
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
    async fn should_return_error_when_sing_in_with_wrong_password(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let email = String::from("user@fiap.com.br");
        let password = String::from("my$ecr3T");
        AuthCommons::craete_customer(&ctx, &email, &password).await;
        let request = build_request(build_request_body(&email, &String::from("wrongPassword")));

        let response = ctx.app.clone().oneshot(request).await?;

        assert_eq!(StatusCode::UNAUTHORIZED, response.status());
        Ok(())
    }
}
