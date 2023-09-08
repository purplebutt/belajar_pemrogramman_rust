use std::fmt::Debug;

const SECRET_KEY: &str = "password321";

#[derive(Clone)]
pub struct Account<T: AccountType + Debug> {
    pub name: String,
    pub active: bool,
    _type: T
}

pub trait AccountType {}

#[derive(Debug)] pub struct Admin;
#[derive(Debug)] pub struct Standard;

impl AccountType for Admin {}
impl AccountType for Standard {}

impl Account<Admin> {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            active: true,
            _type: Admin
        }
    }
    pub fn disable(&self, account: &mut Account<Standard>) {
        account.active = false;
        println!("{} is active:  {}", account.name, account.active);
    }
    pub fn enable(&self, account: &mut Account<Standard>) {
        account.active = true;
        println!("{} is active:  {}", account.name, account.active);
    }
}

impl Account<Standard> {
    pub fn new(name: &str) -> Self {
        Self { 
            name: name.to_string(), 
            active: true,
            _type: Standard
        }
    }
    pub fn make_super(
        self,
        secret_key: &str) -> Result<Account<Admin>, String> {
        // let is_valid = ValidateKeyFromDatabase(secret_key);
        // if is_valid {
        //     // todo
        // }
        // else {
        //     // todo
        // }

        if secret_key == SECRET_KEY {
            let new_super = Account::<Admin>::new(&self.name);
            Ok(new_super)
        }
        else {
            Err(format!("{} is not a valid key!", secret_key))
        }
    }
}

impl<T: AccountType + Debug> Account<T> {
    pub fn change_name(&mut self, new_name: &str) {
        self.name = new_name.into();
    }
    pub fn print(&self) {
        println!("name: {}, is active: {}, type: {:?}", 
            self.name, 
            self.active, 
            self._type
        );
    }
}

