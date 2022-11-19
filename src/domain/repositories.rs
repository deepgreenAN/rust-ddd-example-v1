use crate::domain::Client;
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait ClientRepository {
    fn by_id(&self, id: Uuid) -> Result<Client, String>;
    fn save(&self, client: Client);
    fn all(&self) -> Vec<Client>;
}
