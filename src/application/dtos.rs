use crate::domain::Client;
use std::ops::Index;
use std::slice::SliceIndex;
use uuid::Uuid;

// -------------------------------------------------------------------------------------------------
// ClientDto

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientDto(Client);

impl ClientDto {
    pub fn id(&self) -> Uuid {
        self.0.id()
    }
    pub fn name(&self) -> &str {
        self.0.name()
    }
    pub fn location(&self) -> &str {
        self.0.location()
    }
}

impl From<Client> for ClientDto {
    fn from(client: Client) -> ClientDto {
        ClientDto(client)
    }
}

// -------------------------------------------------------------------------------------------------
// DtoList

#[derive(Debug, PartialEq, Eq)]
pub struct DtoList<T>(Vec<T>);

// vecのメソッドへ委譲
impl<T> DtoList<T> {
    pub fn new(vec: Vec<T>) -> Self {
        Self(vec)
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

// FromIterator
impl<T> FromIterator<T> for DtoList<T> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let vec: Vec<T> = FromIterator::from_iter(iter);
        DtoList(vec)
    }
}

// into_iter，iterについて
impl<T> IntoIterator for DtoList<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> DtoList<T> {
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }
    // pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
    //     self.0.iter_mut()
    // }
}

impl<'a, T> IntoIterator for &'a DtoList<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// impl<'a, T> IntoIterator for &'a mut DtoList<T> {
//     type Item = &'a mut T;
//     type IntoIter = std::slice::IterMut<'a, T>;
//     fn into_iter(self) -> Self::IntoIter {
//         self.iter_mut()
//     }
// }

impl<T, I> Index<I> for DtoList<T>
where
    I: SliceIndex<[T]>,
{
    type Output = <I as SliceIndex<[T]>>::Output;
    fn index(&self, index: I) -> &<Self as Index<I>>::Output {
        &self.0[index]
    }
}

// impl<T, I> IndexMut<I> for DtoList<T>
// where
//     I: SliceIndex<[T]>,
// {
//     fn index_mut(&mut self, index: I) -> &mut <Self as Index<I>>::Output {
//         &mut self.0[index]
//     }
// }

// -------------------------------------------------------------------------------------------------
// test

#[cfg(test)]
mod test {
    use super::{ClientDto, DtoList};
    use crate::domain::Client;
    use fake::{Fake, Faker};
    use std::iter::zip;

    #[test]
    fn create_dto_from_client() {
        let client = Faker.fake::<Client>();
        let client_dto: ClientDto = client.clone().into();
        assert_eq!(client_dto.id(), client.id());
        assert_eq!(client_dto.name(), client.name());
        assert_eq!(client_dto.location(), client.location());
    }

    #[test]
    fn dtolist_like_vec() {
        let clients_num = 10_usize;
        let clients_vec = (0..clients_num)
            .map(|_| Faker.fake::<Client>())
            .collect::<Vec<_>>();

        // collect, iter
        let clients_dto_list = clients_vec
            .iter()
            .cloned()
            .map(Into::into)
            .collect::<DtoList<ClientDto>>();

        for (client, client_dto) in zip(clients_vec.iter(), clients_dto_list.iter()) {
            let client_dto2: &ClientDto = &client.clone().into();
            assert_eq!(client_dto, client_dto2);
        }

        // index
        for i in 0..clients_num {
            let client_dto2: ClientDto = clients_vec[i].clone().into();
            assert_eq!(clients_dto_list[i], client_dto2);
        }

        // into_iter
        for (client, client_dto) in zip(clients_vec.into_iter(), clients_dto_list.into_iter()) {
            let client_dto2: ClientDto = client.clone().into();
            assert_eq!(client_dto, client_dto2);
        }
    }
}
