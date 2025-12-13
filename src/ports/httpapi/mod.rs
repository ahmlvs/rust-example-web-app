use crate::{app::query::get_hello_world::Repository, di::Container};
use axum::{extract::State, routing::get, Router};
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct Server<R>
where
    R: Repository + Send + Sync + 'static,
{
    port: u16,
    container: Arc<Container<R>>,
}

impl<R> Server<R>
where
    R: Repository + Send + Sync + 'static,
{
    pub fn new(port: u16, container: Arc<Container<R>>) -> Self {
        Self { port, container }
    }

    pub async fn run(&self) {
        let app = get_router(self.container.clone());

        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port))
            .await
            .unwrap();

        axum::serve(listener, app).await.unwrap();
    }
}

async fn handler<R>(State(container): State<Arc<Container<R>>>) -> &'static str
where
    R: Repository + Send + Sync + 'static,
{
    container.hello_world_query.execute().await
}

fn get_router<R>(container: Arc<Container<R>>) -> Router
where
    R: Repository + Send + Sync + 'static,
{
    Router::new()
        .route("/hello", get(handler::<R>))
        .with_state(container)
}

#[cfg(test)]
mod tests {
    use crate::app::query::get_hello_world::InMemoryRepository;

    use super::*;
    use axum::{body::Body, http};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    fn setup() -> Arc<Container<InMemoryRepository>> {
        Arc::new(Container::new(InMemoryRepository))
    }

    #[tokio::test]
    async fn test_get_router() {
        // Given
        let container = setup();
        let app = get_router(container);

        // When
        let response = app
            .oneshot(
                http::Request::builder()
                    .uri("/hello")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Then
        assert_eq!(response.status(), 200);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Hello, world!");
    }

    #[tokio::test]
    async fn not_found() {
        // Given
        let container = setup();
        let app = get_router(container);

        // When
        let response = app
            .oneshot(
                http::Request::builder()
                    .uri("/not_found")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Then
        assert_eq!(response.status(), 404);
    }
}
