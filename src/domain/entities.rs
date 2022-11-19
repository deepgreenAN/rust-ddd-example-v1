use uuid::Uuid;

#[cfg(test)]
use fake::{Dummy, Fake};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Dummy))]
pub struct Client {
    id: Uuid,
    name: String,
    location: String,
}

impl Client {
    pub fn new(name: String, location: String) -> Self {
        let id = Uuid::new_v4();
        Self { id, name, location }
    }
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn location(&self) -> &str {
        &self.location
    }
    pub fn edit(&mut self, name: String, location: String) {
        self.name = name;
        self.location = location;
    }
}

#[cfg(test)]
mod test {
    use fake::{Fake, Faker};

    use super::Client;
    #[test]
    fn create_client() {
        let name = Faker.fake::<String>();
        let location = Faker.fake::<String>();

        let client = Client::new(name.clone(), location.clone());

        assert_eq!(&name, client.name());
        assert_eq!(&location, client.location());
    }

    #[test]
    fn edit_client() {
        let mut client = Faker.fake::<Client>();

        let id = client.id();
        let new_name = Faker.fake::<String>();
        let new_location = Faker.fake::<String>();

        assert_ne!(client.name(), &new_name);
        assert_ne!(client.location(), &new_location);

        client.edit(new_name.clone(), new_location.clone());

        assert_eq!(client.id(), id);
        assert_eq!(client.name(), &new_name);
        assert_eq!(client.location(), &new_location);
    }
}
