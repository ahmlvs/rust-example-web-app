use crate::id_provider::IDProvider;

pub trait CreateShortUrlRepository {
    fn save(&self, full_url: String, id: String) -> Result<(), String>;
}

pub struct CreateShortUrlCommand<I, R>
where
    I: IDProvider,
    R: CreateShortUrlRepository,
{
    id_provider: I,
    repo: R,
}

impl<I, R> CreateShortUrlCommand<I, R>
where
    I: IDProvider,
    R: CreateShortUrlRepository,
{
    pub fn new(id_provider: I, repo: R) -> Self {
        Self { id_provider, repo }
    }

    pub async fn execute(&self, full_url: String) -> Result<String, String> {
        let _id = self.id_provider.provide();

        self.repo.save(full_url, _id.clone())?;

        Ok(_id)
    }
}

#[cfg(test)]
mod tests {
    use dashmap::DashMap;

    use crate::{adapters::inmemory::InMemoryRepository, id_provider};
    use std::sync::Arc;

    use super::*;

    #[tokio::test]
    async fn get_short_url() {
        // Given
        let id_provider = id_provider::FakeIDProvider::new("123".to_owned());
        let store = Arc::new(DashMap::new());
        let repo = InMemoryRepository::new(store);
        let command = CreateShortUrlCommand::new(id_provider, repo);

        // When
        let result = command.execute("https://google.com".to_owned()).await;

        // Then
        assert_ne!(result, Ok("".to_owned()));
    }

    #[tokio::test]
    async fn get_two_different_short_url() {
        // Given
        let idp = id_provider::NanoIDProvider;
        let store = Arc::new(DashMap::new());
        let repo = InMemoryRepository::new(store);
        let command = CreateShortUrlCommand::new(idp, repo);

        // When
        let result1 = command.execute("https://google.com".to_owned()).await;
        let result2 = command.execute("https://google.com".to_owned()).await;

        // Then
        assert_ne!(result1, result2);
    }

    #[tokio::test]
    async fn after_save_store_should_have_one_item() {
        // Given
        let idp = id_provider::NanoIDProvider;
        let store = Arc::new(DashMap::new());
        let repo = InMemoryRepository::new(store.clone());
        let command = CreateShortUrlCommand::new(idp, repo);

        // When
        let id = command
            .execute("https://google.com".to_owned())
            .await
            .unwrap();

        // Then
        assert_eq!(store.len(), 1);
        let full_url = store.get(&id).unwrap();
        assert_eq!(full_url.value(), "https://google.com");
    }
}
