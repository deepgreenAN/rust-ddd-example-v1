use crate::domain::repositories::ClientRepository;
use std::rc::Rc;

pub trait Handler<T: ClientRepository> {
    type Request;
    type Output;
    fn new(client_repository: Rc<T>) -> Self;
    fn execute(&self, request: Self::Request) -> Self::Output;
}
