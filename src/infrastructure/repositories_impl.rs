use crate::domain::{Client, ClientRepository};
use std::cell::RefCell;
use std::collections::HashMap;
use uuid::Uuid;

pub struct InMemoryClientRepository {
    clients: RefCell<HashMap<Uuid, Client>>,
}

impl InMemoryClientRepository {
    pub fn new() -> Self {
        Self {
            clients: RefCell::new(HashMap::new()),
        }
    }
}

impl ClientRepository for InMemoryClientRepository {
    fn by_id(&self, id: Uuid) -> Result<Client, String> {
        match self.clients.borrow().get(&id) {
            Some(client) => Ok(client.clone()),
            None => Err("No client found for given ID".to_string()),
        }
    }
    fn save(&self, client: Client) {
        self.clients.borrow_mut().insert(client.id(), client);
    }
    fn all(&self) -> Vec<Client> {
        let clients = self.clients.borrow();
        let mut vec_clients: Vec<Client> = Vec::with_capacity(clients.len());
        for client in clients.values() {
            vec_clients.push(client.clone());
        }
        vec_clients
    }
}

impl InMemoryClientRepository {
    pub fn new_with_clients() -> Self {
        let repository = Self {
            clients: RefCell::new(HashMap::new()),
        };
        repository.save(Client::new("Taro".to_string(), "Tokyo".to_string()));
        repository.save(Client::new("Jiro".to_string(), "Tokyo".to_string()));
        repository
    }
}

#[cfg(test)]
mod test {
    use super::{Client, ClientRepository, InMemoryClientRepository};
    use assert_matches::assert_matches;
    use fake::{Fake, Faker};

    fn check_clients<T: ClientRepository>(repository: &T) {
        let empty_vec_clients: Vec<Client> = Vec::new();
        assert_eq!(empty_vec_clients, repository.all());

        let mut vec_clients: Vec<Client> = Vec::new();
        let client_number = 10;

        for _ in 0..client_number {
            let client = Client::new(Faker.fake::<String>(), Faker.fake::<String>());
            vec_clients.push(client.clone());
            repository.save(client);
        }

        // by_id?????????????????????
        for client in vec_clients.iter() {
            assert_eq!(*client, repository.by_id(client.id()).unwrap());
        }

        // by_id?????????????????????
        assert_matches!(repository.by_id(Faker.fake()), Err(_));

        // all???vec_clients?????????
        vec_clients.sort_by_key(|client| client.id());

        let mut all_clients = repository.all();
        all_clients.sort_by_key(|client| client.id());
        assert_eq!(vec_clients, all_clients);
    }

    #[test]
    fn check_repositories() {
        let repository = InMemoryClientRepository::new();
        check_clients(&repository);
    }
}
