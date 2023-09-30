use std::io::{self, Write};
use std::fs::{self,File};
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
// use::std::io::{self, Write};

struct Person {
    name: String,
    age: i16,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
struct BasicAccountMembers {
    account_holder: String,
    id: i16,
}

impl BasicAccountMembers {
    fn increment_id(&mut self) {
        self.id += 1;
    }
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
}

impl Account {
    fn deposit(&mut self) {
        print!("Enter deposit amount: ");
        let _ = io::stdout().flush();
        let deposit_amount = get_user_float();
        self.balance += deposit_amount;
    }

    fn withdraw(&mut self) {
        print!("Enter withdraw amount: ");
        let _ = io::stdout().flush();
        let withdraw_amount = get_user_float();
        self.balance -= withdraw_amount;
    }


}
fn save_account_to_json(data: &Account) -> io::Result<()> {
    let json_string = serde_json::to_string(data)?;
    let file_name = format!("AccountJsonfiles\\{}.json", data.account_holder);
    let mut file = File::create(file_name)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}

fn save_id_struct_to_json(data: &BasicAccountMembers) -> io::Result<()> {
    let json_string = serde_json::to_string(data)?;
    let file_name = format!("account_id\\{}.json", data.account_holder);
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
     
    let _ = io::stdout().flush().unwrap();
    
    io::stdin()
        .read_line(&mut user_input_string)
        .expect("failed to read input");
    let trimmed_input = user_input_string.trim().to_string();
   
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

fn create_account(account_id: i16) -> Account {
    // let person: Person = create_person();
    print!("Enter account holders name: ");
    let _ = io::stdout().flush();
    let name:&str = &get_user_string();
    print!("enter account balance: ");
    let _ = io::stdout().flush();
    let _balance: f32 = get_user_float();
    let account = Account {
        id: account_id,
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

fn load_account_id_tracker_struct_from_json() -> io::Result<BasicAccountMembers> {
    // let mut current_account_id = Vec::new();
    let full_path = String::from("account_id/current_id.json");
    let mut file = File::open(full_path)?;
    let mut json_string = String::new();
    file.read_to_string(&mut json_string)?;
    let current_id: BasicAccountMembers = serde_json::from_str(&json_string)?;
    Ok(current_id)
}

fn load_account_id_tracker_to_vector() -> io::Result<Vec<BasicAccountMembers>> {
    let mut current_account_id = Vec::new();
    match load_account_id_tracker_struct_from_json() {
        Ok(current_id_struct) => {
            println!("loaded existing json");
            current_account_id.push(current_id_struct);
        }
        Err(err) => {
            println!("No existing json, loaded default json");
            let basic_account_members: BasicAccountMembers = BasicAccountMembers { account_holder:String::from("current_id"),id: 0 };
            current_account_id.push(basic_account_members);
        }
    }
    Ok(current_account_id)
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


fn find_account(accounts: &mut Vec<Account>) -> Option<&mut Account> {
    println!("Enter account id to search for: ");
    let _ = io::stdout().flush();
    let account_id = get_user_integer();
    for account in accounts.iter_mut() {
        if account.id == account_id {
            return Some(account);
        }
    }
    None
}

fn program_startup() ->(Vec<Account>,Vec<BasicAccountMembers>) {
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

    let account_id_results = load_account_id_tracker_to_vector();

    let current_id = match account_id_results {
        Ok(current_id) => {
            current_id
        }
        Err(err) => {
            eprintln!("Error loading id struct from JSON: {}", err);
            Vec::new()
        }
    };

    let return_tuple = (accounts, current_id);
    return return_tuple;

}

fn delete_account(accounts: &mut Vec<Account> ) -> std::io::Result <()> {
    print!("Enter name of account holder to delete: ");
    let _ = io::stdout().flush();
    let account_holder_to_delete = get_user_string();
    // remove account from Accounts Struct
    accounts.retain(|account| account.account_holder != account_holder_to_delete);
    // remove json file
    let file_name_to_delete = format!("AccountJsonfiles\\{}.json", account_holder_to_delete);
    let _ = fs::remove_file(file_name_to_delete);

    Ok(())

}
fn menu(accounts: &mut Vec<Account>, current_id: &mut Vec<BasicAccountMembers>) {
    // let mut current_account_id = load_account_id_tracker_to_vector();
    
    let mut user_selection: i16;
    loop {
        println!("1. Create New Account.");
        println!("2. lookup Account.");
        println!("3. Delete Account.");
        println!("4. Make deposit.");
        println!("5. Make withdrawal.");
        println!("6. Display all accounts.");
        println!("0. Quit.");
    
        user_selection = get_user_integer();
        
        // quit
        if user_selection == 0 {
            break
        // create new account
        } else if user_selection == 1 {
            let account: Account = create_account(current_id[0].id);
            accounts.push(account);
            current_id[0].increment_id();
            let _ = save_id_struct_to_json(&current_id[0]);
        // display account details
        } else if user_selection == 2 {
            if let Some(account_to_list_details) = find_account(accounts) {
                println!("account holder: {}", account_to_list_details.account_holder);
                println!("account id: {}", account_to_list_details.id);
                println!("account balance: {}", account_to_list_details.balance);
            }
        // delete account
        } else if user_selection == 3 {
            let _ = delete_account(accounts);
        // deposit
        } else if user_selection == 4 { 
            if let Some(account_to_modify) = find_account(accounts) {
                account_to_modify.deposit();
                let _ = save_account_to_json(&account_to_modify);
            }
        // withdraw
        } else if user_selection == 5 {
            if let Some(account_to_modify) = find_account(accounts) {
                account_to_modify.withdraw();
                let _ = save_account_to_json(&account_to_modify);
            }
        // display all account id and holder
        } else if user_selection == 6 { 
            for account in &mut *accounts {
                println!("Account ID: {} Account holder: {} ", account.id, account.account_holder);
                println!()
            }
        // error
        } else {
            println!("invalid input.");
        }
    }

}
fn main() {
    let (mut accounts, mut current_id) = program_startup();

    let _ = menu(&mut accounts, &mut current_id);

    // To Do
    // enable deletion of accounts from list, and from file *DONE*
    // implement struct methods
    



    
    
    
}

