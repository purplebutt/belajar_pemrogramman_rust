const SECRET_KEY: &str = "password321";

pub struct Account<T: AccountType> {
    pub name: String,
    pub active: bool,
    _type: T
}

pub trait AccountType {}

pub struct Admin;
pub struct Standard;

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
}

impl Account<Standard> {
    pub fn new(name: &str) -> Self {
        Self { 
            name: name.to_string(), 
            active: true,
            _type: Standard
        }
    }
    pub fn make_super(account: Account<Standard>, secret_key: &str) -> Result<Account<Admin>, String> {
        if secret_key == SECRET_KEY {
            let new_super = Account::<Admin>::new(&account.name);
            Ok(new_super)
        }
        else {
            Err(format!("{} is not a valid key!", secret_key))
        }
    }
}

impl<T: AccountType> Account<T> {
    pub fn change_name(&mut self, new_name: &str) {
        self.name = new_name.into()
    }
}

pub trait AdminTasks {
    fn disable(&self, account: &mut Account<Standard>);
    fn enable(&self, account: &mut Account<Standard>);
}
pub trait StandardTasks {
    fn request_as_admin(&self, admin: Account<Admin>);
}
pub trait GeneralTasks {
    fn set_name(&mut self, new_name: &str);
}

impl<T: AccountType> GeneralTasks for Account<T> {
    fn set_name(&mut self, new_name: &str) {
        self.name = new_name.into();
    } 
}

impl AdminTasks for Account<Admin> {
    fn disable(&self, account: &mut Account<Standard>) {
        account.active = false;
        println!("{} is active:  {}", account.name, account.active);
    }
    fn enable(&self, account: &mut Account<Standard>) {
        println!("{} is active:  {}", account.name, account.active);
    }
}

