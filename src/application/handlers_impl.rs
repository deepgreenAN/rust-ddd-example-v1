use crate::application::dtos::{ClientDto, DtoList};
use crate::application::requests::{
    CreateClientUseCaseRequest, EditClientUseCaseRequest, GetClientUseCaseRequest, NoneRequest,
};
use crate::application::Handler;
use crate::domain::{Client, ClientRepository};
use std::rc::Rc;

// -------------------------------------------------------------------------------------------------

pub struct CreateClientUseCaseHandler<T: ClientRepository> {
    client_repo: Rc<T>,
}

impl<T: ClientRepository> Handler<T> for CreateClientUseCaseHandler<T> {
    type Request = CreateClientUseCaseRequest;
    type Output = ();
    fn new(client_repo: Rc<T>) -> Self {
        Self { client_repo }
    }
    fn execute(&self, request: Self::Request) -> Self::Output {
        let id = self.client_repo.next_identity();
        let client = Client::new(id, request.name, request.location);
        self.client_repo.save(client);
    }
}

// -------------------------------------------------------------------------------------------------

pub struct GetClientUseCaseHandler<T: ClientRepository> {
    client_repo: Rc<T>,
}

impl<T: ClientRepository> Handler<T> for GetClientUseCaseHandler<T> {
    type Request = GetClientUseCaseRequest;
    type Output = Result<ClientDto, String>;
    fn new(client_repo: Rc<T>) -> Self {
        Self { client_repo }
    }
    fn execute(&self, request: Self::Request) -> Self::Output {
        self.client_repo
            .by_id(request.id)
            .map(|client| client.into())
    }
}

// -------------------------------------------------------------------------------------------------

pub struct GetAllClientUseCaseHandler<T: ClientRepository> {
    client_repo: Rc<T>,
}

impl<T: ClientRepository> Handler<T> for GetAllClientUseCaseHandler<T> {
    type Request = NoneRequest;
    type Output = DtoList<ClientDto>;
    fn new(client_repo: Rc<T>) -> Self {
        Self { client_repo }
    }
    fn execute(&self, _: Self::Request) -> Self::Output {
        self.client_repo
            .all()
            .into_iter()
            .map(Into::into)
            .collect::<DtoList<ClientDto>>()
    }
}

// -------------------------------------------------------------------------------------------------

pub struct EditClientUseCaseHandler<T: ClientRepository> {
    client_repo: Rc<T>,
}

impl<T: ClientRepository> Handler<T> for EditClientUseCaseHandler<T> {
    type Request = EditClientUseCaseRequest;
    type Output = Result<(), String>;
    fn new(client_repo: Rc<T>) -> Self {
        Self { client_repo }
    }
    fn execute(&self, request: Self::Request) -> Self::Output {
        let mut client = self.client_repo.by_id(request.id)?;
        client.edit(request.name, request.location);
        self.client_repo.save(client);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{
        CreateClientUseCaseHandler, EditClientUseCaseHandler, GetAllClientUseCaseHandler,
        GetClientUseCaseHandler, Handler,
    };
    use crate::application::dtos::{ClientDto, DtoList};
    use crate::application::requests::{
        CreateClientUseCaseRequest, EditClientUseCaseRequest, GetClientUseCaseRequest, NoneRequest,
    };
    use assert_matches::assert_matches;
    use fake::faker::{address::en::CityName, name::en::Name};
    use fake::{Fake, Faker};
    use mockall::predicate;
    use std::rc::Rc;

    use crate::domain::repositories::MockClientRepository;
    use crate::domain::Client;

    #[test]
    fn create_client_use_case_handler_execute() {
        let client = Faker.fake::<Client>();
        let id = client.id();

        let mut mock_repo = MockClientRepository::new();

        mock_repo.expect_next_identity().times(1).return_const(id);

        mock_repo
            .expect_save()
            .withf(move |inner_client| inner_client.id() == id)
            .times(1)
            .return_const(());

        let create_client_use_case_handler = CreateClientUseCaseHandler::new(Rc::new(mock_repo));
        create_client_use_case_handler.execute(CreateClientUseCaseRequest::new(
            Name().fake::<String>(),
            CityName().fake::<String>(),
        ))
    }

    #[test]
    fn get_client_use_case_handler_execute_ok() {
        let client = Faker.fake::<Client>();
        let client_dto: ClientDto = client.clone().into();
        let id = client.id();

        let mut mock_repo = MockClientRepository::new();

        mock_repo
            .expect_by_id()
            .with(predicate::eq(id))
            .times(1)
            .return_const(Ok(client));

        let get_client_use_case_handler = GetClientUseCaseHandler::new(Rc::new(mock_repo));
        let res_client_dto = get_client_use_case_handler.execute(GetClientUseCaseRequest::new(id));
        assert_eq!(res_client_dto, Ok(client_dto))
    }

    #[test]
    fn get_client_use_case_hndler_execute_err() {
        let client = Faker.fake::<Client>();
        let id = client.id();

        let mut mock_repo = MockClientRepository::new();

        mock_repo
            .expect_by_id()
            .with(predicate::eq(id))
            .times(1)
            .return_const(Err("some_error".to_string()));

        let get_client_use_case_handler = GetClientUseCaseHandler::new(Rc::new(mock_repo));
        let res_client_dto = get_client_use_case_handler.execute(GetClientUseCaseRequest::new(id));

        assert_matches!(res_client_dto, Err(_));
    }

    #[test]
    fn get_all_clients_use_case_handler_execute() {
        let clients: Vec<Client> = vec![Faker.fake(), Faker.fake(), Faker.fake()];
        let client_dtos = clients
            .iter()
            .cloned()
            .map(Into::into)
            .collect::<DtoList<ClientDto>>();

        let mut mock_repo = MockClientRepository::new();
        mock_repo.expect_all().times(1).return_const(clients);

        let get_all_clients_use_case_handler = GetAllClientUseCaseHandler::new(Rc::new(mock_repo));

        let client_dtos2 = get_all_clients_use_case_handler.execute(NoneRequest);
        assert_eq!(client_dtos, client_dtos2);
    }

    #[test]
    fn edit_client_use_case_handler_execute_ok() {
        let client = Faker.fake::<Client>();
        let new_name = Name().fake::<String>();
        let new_location = Name().fake::<String>();

        let expeted_client = Client::new(client.id(), new_name.clone(), new_location.clone());
        let mut mock_repo = MockClientRepository::new();
        mock_repo
            .expect_by_id()
            .with(predicate::eq(client.id()))
            .times(1)
            .return_const(Ok(client.clone()));

        mock_repo
            .expect_save()
            .with(predicate::eq(expeted_client))
            .times(1)
            .return_const(());

        let edit_client_use_case_handler = EditClientUseCaseHandler::new(Rc::new(mock_repo));

        let res = edit_client_use_case_handler.execute(EditClientUseCaseRequest::new(
            client.id(),
            new_name,
            new_location,
        ));
        assert_eq!(res, Ok(()));
    }

    #[test]
    fn edit_client_use_case_handler_execute_err() {
        let client = Faker.fake::<Client>();
        let new_name = Name().fake::<String>();
        let new_location = Name().fake::<String>();

        let mut mock_repo = MockClientRepository::new();
        mock_repo
            .expect_by_id()
            .with(predicate::eq(client.id()))
            .times(1)
            .return_const(Err("Some Error".to_string()));

        mock_repo.expect_save().times(0).return_const(());

        let edit_client_use_case_handler = EditClientUseCaseHandler::new(Rc::new(mock_repo));

        let res = edit_client_use_case_handler.execute(EditClientUseCaseRequest::new(
            client.id(),
            new_name,
            new_location,
        ));
        assert_matches!(res, Err(_));
    }
}
