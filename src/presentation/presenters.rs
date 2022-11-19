use std::fmt::Display;

use crate::application::dtos::{ClientDto, DtoList};

impl Display for ClientDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Client #{}: {}, from {}",
            self.id().hyphenated(),
            self.name(),
            self.location()
        )
    }
}

impl Display for DtoList<ClientDto> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return writeln!(f, "No clients");
        }

        writeln!(f, "Client list")?;
        writeln!(f, "----------------------------------------\n")?;

        for client_dto in self.iter() {
            writeln!(f, "{}", client_dto)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::application::dtos::{ClientDto, DtoList};
    use crate::domain::Client;
    use fake::faker::{address::en::CityName, name::en::Name};
    use fake::Fake;
    use std::fmt::Write;

    #[test]
    fn client_dto_print() {
        let client_dto: ClientDto = Client::new(Name().fake(), CityName().fake()).into();
        assert_eq!(
            client_dto.to_string(),
            format!(
                "Client #{}: {}, from {}",
                client_dto.id().hyphenated(),
                client_dto.name(),
                client_dto.location()
            )
        );
    }

    #[test]
    fn client_dto_list_empty_print() {
        let client_dto_list: DtoList<ClientDto> = DtoList::new(Vec::new());
        assert_eq!(client_dto_list.to_string(), "No clients\n");
    }

    #[test]
    fn client_dto_list_print() {
        let dto_lists = (0..2)
            .map(|_| Client::new(Name().fake(), CityName().fake()).into())
            .collect::<DtoList<ClientDto>>();

        let mut expected_string = String::new();
        writeln!(expected_string, "Client list").unwrap();
        writeln!(
            expected_string,
            "----------------------------------------\n"
        )
        .unwrap();

        for client_dto in dto_lists.iter() {
            writeln!(expected_string, "{}", client_dto).unwrap();
        }

        assert_eq!(expected_string, dto_lists.to_string());
    }
}
