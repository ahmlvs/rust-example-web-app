pub mod command;
pub mod query;

#[cfg(test)]
mod tests {
    use crate::adapters::inmemory::InMemoryRepository;
    use dashmap::DashMap;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_create_and_get_url() {
        // Given
        let store = Arc::new(DashMap::new());
        let repo = InMemoryRepository::new(store.clone());
        let create_command = crate::app::command::create_short_url::CreateShortUrlCommand::new(
            crate::id_provider::FakeIDProvider::new("123".to_owned()),
            repo.clone(),
        );
        let get_query = crate::app::query::get_full_url::GetFullUrlQuery::new(repo);

        // When
        let result1 = create_command.execute("https://google.com").await;
        let result2 = get_query.execute(&result1.unwrap()).await.unwrap();
        // Then
        assert_eq!(result2, "https://google.com/".to_owned());
    }
}
