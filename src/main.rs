mod application;
mod domain;
mod infrastructure;
mod presentation;

use application::handlers_impl::{
    CreateClientUseCaseHandler, EditClientUseCaseHandler, GetAllClientUseCaseHandler,
    GetClientUseCaseHandler,
};
use application::requests::{
    CreateClientUseCaseRequest, EditClientUseCaseRequest, GetClientUseCaseRequest, NoneRequest,
};
use application::Handler;
use clap::Parser;
use dialoguer::{Input, Select};
use infrastructure::InMemoryClientRepository;
use std::error::Error;
use std::rc::Rc;
use uuid::Uuid;

fn app<T: domain::ClientRepository>(repository: Rc<T>) -> Result<(), Box<dyn Error>> {
    let crate_client_use_case_handler = CreateClientUseCaseHandler::new(Rc::clone(&repository));
    let edit_client_use_case_handler = EditClientUseCaseHandler::new(Rc::clone(&repository));
    let get_client_use_case_handler = GetClientUseCaseHandler::new(Rc::clone(&repository));
    let get_all_clients_use_case_handler = GetAllClientUseCaseHandler::new(Rc::clone(&repository));

    let select_vec = vec![
        "終了 0",
        "全てのクライアントをリストで表示 1",
        "クライアントを呼び出し 2",
        "クライアントを作成 3",
        "クライアントを編集 4",
    ];

    'app: loop {
        println!(); // 空行

        let select = Select::new()
            .with_prompt("コマンドを選択してください")
            .items(&select_vec)
            .interact()?;

        println!(); // 空行

        match select {
            1 => {
                let clients = get_all_clients_use_case_handler.execute(NoneRequest);
                println!("{}", clients);
            }
            2 => {
                let input_id_string = Input::<'_, String>::new()
                    .with_prompt("検索するIDを入力してください >")
                    .validate_with(|input: &String| -> Result<(), &str> {
                        Uuid::parse_str(input)
                            .map(|_| ())
                            .map_err(|_| "id parse err")
                    })
                    .interact()?;

                let input_id = Uuid::parse_str(&input_id_string)?;

                let client =
                    get_client_use_case_handler.execute(GetClientUseCaseRequest::new(input_id));
                match client {
                    Ok(client) => {
                        println!("{}", client);
                    }
                    Err(err) => {
                        eprintln!("Error:{}", err);
                    }
                }
            }
            3 => {
                let input_name: String = Input::new()
                    .with_prompt("作成したいクライアントの名前を入力してください >")
                    .interact()?;

                let input_location: String = Input::new()
                    .with_prompt("作成したいクライアントの出身地を入力してください >")
                    .interact()?;

                crate_client_use_case_handler
                    .execute(CreateClientUseCaseRequest::new(input_name, input_location));
                println!("クライアントが作成されました");
            }
            4 => {
                let input_id_string = Input::<'_, String>::new()
                    .with_prompt("編集するIDを入力してください >")
                    .validate_with(|input: &String| -> Result<(), &str> {
                        Uuid::parse_str(input)
                            .map(|_| ())
                            .map_err(|_| "id parse err")
                    })
                    .interact()?;

                let input_id = Uuid::parse_str(&input_id_string)?;
                let input_name: String = Input::new()
                    .with_prompt("新しい名前を入力してください >")
                    .interact()?;
                let input_location: String = Input::new()
                    .with_prompt("新しい出身地を入力してください >")
                    .interact()?;

                let res = edit_client_use_case_handler.execute(EditClientUseCaseRequest::new(
                    input_id,
                    input_name,
                    input_location,
                ));
                match res {
                    Ok(_) => {
                        println!("クライアントを編集しました．");
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                    }
                }
            }
            0 => {
                println!("終了します");
                break 'app;
            }
            _ => {
                eprintln!("未定義のオプションが選択されました");
                break 'app;
            }
        }
    }
    Ok(())
}

#[derive(Parser)]
struct Cli {
    /// with some samples
    #[arg(long)]
    sample: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let repository = match cli.sample {
        true => Rc::new(InMemoryClientRepository::new_with_clients()),
        false => Rc::new(InMemoryClientRepository::new()),
    };

    app(repository)?;
    Ok(())
}
