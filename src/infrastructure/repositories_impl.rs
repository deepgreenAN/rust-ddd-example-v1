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
            None => Err("No client found for geven ID".to_string()),
        }
    }
    fn save(&self, client: Client) {
        self.clients.borrow_mut().insert(client.id(), client);
    }
    fn next_identity(&self) -> Uuid {
        Uuid::new_v4()
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
        let id_1 = repository.next_identity();
        repository.save(Client::new(id_1, "Taro".to_string(), "Tokyo".to_string()));
        let id_2 = repository.next_identity();
        repository.save(Client::new(id_2, "Jiro".to_string(), "Tokyo".to_string()));
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
            let id = repository.next_identity();
            let client = Client::new(id, Faker.fake::<String>(), Faker.fake::<String>());
            vec_clients.push(client.clone());
            repository.save(client);
        }

        // by_idで取得して比較
        for client in vec_clients.iter() {
            assert_eq!(*client, repository.by_id(client.id()).unwrap());
        }

        // by_idでエラーのとき
        assert_matches!(repository.by_id(repository.next_identity()), Err(_));

        // allとvec_clientsを比較
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
