use uuid::Uuid;

pub struct CreateClientUseCaseRequest {
    pub name: String,
    pub location: String,
}

impl CreateClientUseCaseRequest {
    pub fn new(name: String, location: String) -> Self {
        Self { name, location }
    }
}

pub struct GetClientUseCaseRequest {
    pub id: Uuid,
}

impl GetClientUseCaseRequest {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

pub struct EditClientUseCaseRequest {
    pub id: Uuid,
    pub name: String,
    pub location: String,
}

impl EditClientUseCaseRequest {
    pub fn new(id: Uuid, name: String, location: String) -> Self {
        Self { id, name, location }
    }
}

pub struct NoneRequest;
