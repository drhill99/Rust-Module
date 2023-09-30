use std::io::{self, Write};
use std::fs::{self,File};
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
// use::std::io::{self, Write};

struct Person {
    name: String,
    age: i16,
}

// impl Person {
//     fn new(age: u32) -> Person {
//         Person {
//             age
//         }
//     }
// }
#[derive(Serialize, Deserialize, Clone, Debug)]
// #[derive(Debug)]
struct Account {
    id: i16,
    account_holder: String,
    balance: f32,
    // transaction_history: Vec<String>,
}
fn save_account_to_json(data: &Account) -> io::Result<()> {
    let json_string = serde_json::to_string(data)?;
    let file_name = format!("AccountJsonfiles\\{}.json", data.account_holder);
    let mut file = File::create(file_name)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}


fn get_user_integer() -> i16 {
    let mut _int: i16 = 0;
    let mut user_input_int = String::new();
    io::stdin()
        .read_line(&mut user_input_int)
        .expect("failed to read input");
    let trimmed_input = user_input_int.trim();  
    let parsed_user_input_age: Result<i16,std::num::ParseIntError> = trimmed_input.parse();
    match parsed_user_input_age {
        Ok(parsed_number) => {
            // println!("parsed_number: {}", parsed_number);
            _int = parsed_number;
        }
        Err(_i16) => {
            println!("failed to parse the string to integer.");
        }
    }    
    return _int;
}

fn get_user_float() -> f32 {
    let mut _flt: f32 = 0.0;
    let mut user_input_flt = String::new();
    io::stdin()
        .read_line(&mut user_input_flt)
        .expect("failed to read input");
    let trimmed_input = user_input_flt.trim();  
    let parsed_user_input_flt: Result<f32,std::num::ParseFloatError> = trimmed_input.parse();
    match parsed_user_input_flt {
        Ok(parsed_number) => {
            // println!("parsed_number: {}", parsed_number);
            _flt = parsed_number;
        }
        Err(_f32) => {
            println!("failed to parse the string to integer.");
        }
    }    
    return _flt;
}


fn get_user_string() -> String {
    let mut user_input_string = String::new();
    // print!("Enter name: ");
    // flush io buffer to execute print! 
    let _ = io::stdout().flush().unwrap();
    // take user input to define Person.name
    io::stdin()
        .read_line(&mut user_input_string)
        .expect("failed to read input");
    let trimmed_input = user_input_string.trim().to_string();
    // let trimmed_string: String = trimmed_input.to_string();
    trimmed_input
}

fn create_person() -> Person {
    let name:&str = &get_user_string();
    print!("Enter age: ");
    let _ = io::stdout().flush();
    let _age: i16 = get_user_integer();
    let person = Person {
        name:String::from(name),
        age:_age,
    };

    return person;
}

fn create_account() -> Account {
    // let person: Person = create_person();
    print!("Enter account holders name: ");
    let _ = io::stdout().flush();
    let name:&str = &get_user_string();
    print!("enter account balance: ");
    let _ = io::stdout().flush();
    let _balance: f32 = get_user_float();
    let account = Account {
        id: 0,
        account_holder:String::from(name),
        balance: _balance,
    };
    let _ = save_account_to_json(&account);
    return account;    
}

fn load_account_from_json(file_name: String) -> io::Result<Account> {
    println!("loading account from json.......");
    let full_path = format!("AccountJsonFiles/{}", file_name);
    let mut file = File::open(full_path)?;

    let mut json_string = String::new();
    file.read_to_string(&mut json_string)?;

    let account: Account = serde_json::from_str(&json_string)?;

    Ok(account)
}

fn generate_list_of_account_jsons() ->io::Result<Vec<String>> {
    let mut json_files = Vec::new();

    for entry in fs::read_dir("AccountJsonFiles")? {
        let entry = entry?;
        let path = entry.path();
        if let Some(extension) = path.extension() {
            if extension == "json" {
                if let Some(file_name) = path.file_name(){
                    if let Some(file_str) = file_name.to_str() {
                        json_files.push(file_str.to_string());
                    }
                }
            }
        }
    }
    
    Ok(json_files)
}

fn load_all_json_files() -> io::Result<Vec<Account>> {
    let mut accounts = Vec::new();
    for file in generate_list_of_account_jsons()? {
        println!("{}", file);
        match load_account_from_json(file) {
            Ok(loaded_account) => {
                let account1 = loaded_account.clone();
                accounts.push(account1);
            }
            Err(err) => eprintln!("Error loading account from JSON: {}", err),
        }
    }

    Ok(accounts)
}
fn find_account(accounts: Vec<Account>) -> Account{
        // iterage through vector of accounts to find target account based on struct member data
        fn iterate_through_accounts(accounts: &[Account]) -> Option<Account> {
            print!("Account ID to search for: ");
            let _ = io::stdout().flush();
            let account_id = get_user_integer();
            // accounts.iter().find(|account| account.id==account_id).clone();
            if let Some(destination_account) = accounts.iter().find(|account| account.id == account_id){
                // let return_account = destination_account.clone();
                // return return_account;
                Some(destination_account.clone())
            } else {
                None
            }
        }
    
    let find_account_result = iterate_through_accounts(&accounts);
    let mut account: Account = Account { id: 0, account_holder: String::new(), balance:0.0 };
    match find_account_result {
        Some(target_account) => {
            account = target_account;
        }
        None => {
            println!("Account not found");
        }
    }
    return account;
}

fn program_startup() -> Vec<Account> {
    let account_results = load_all_json_files();
    
    let accounts = match account_results {
        Ok(accounts) => {
            accounts
        }
        Err(err) => {
            eprintln!("Error loading accounts from JSON: {}", err);
            Vec::new()  // Return an empty vector or handle the error in another way
        }
    };  
    return accounts;
}

fn main() {
    let mut accounts = program_startup();
    let account = find_account(accounts.clone());
    let mut user_selection: i16 = -1;
    loop {
        println!("1. Create New Account.");
        println!("2. lookup Account.");
        println!("3. Print Account Details.");
        println!("0. Quit.");

        user_selection = get_user_integer();

        if user_selection == 0 {
            break
        } else if user_selection == 1 {
            let account: Account = create_account();
            accounts.push(account);
        } else if user_selection == 2 {
            break
        } else if user_selection == 3 {
            break
        } else {
            println!("invalid input.");
            break
        }
    }
    // for account in accounts {
    //     println!("account holder: {}", account.account_holder);
    //     println!("account id: {}", account.id);
    //     println!("account balance: {}", account.balance);
    // }
    
    // println!("name: {}", accounts[0].account_holder);
    // println!("id: {}", accounts[0].id);
    // println!("balance: {}", accounts[0].balance);
    
    // println!("name: {}", accounts[1].account_holder);
    // println!("id: {}", accounts[1].id);
    // println!("balance: {}", accounts[1].balance);


    println!("name: {}", account.account_holder);
    println!("id: {}", account.id);
    println!("balance: {}", account.balance);
    



    
    
    
}

