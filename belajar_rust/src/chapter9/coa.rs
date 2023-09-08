use std::collections::HashMap;

pub enum Normal { DEBIT, CREDIT }
impl Default for Normal {
    fn default() -> Self { Self::DEBIT }
}

#[derive(Debug)]
pub enum AccountGroup {
    ASSET = 1,
    LIABILITY = 2,
    EQUITY = 3,
    REVENUE = 4,
    EXPENSE = 5,
    INCOME = 8
}
impl Default for AccountGroup {
    fn default() -> Self { Self::ASSET }
}

pub enum Report { IS, ES, BS }
impl Default for Report {
    fn default() -> Self { Self::BS } 
}

#[derive(Default)]
pub struct Account {
    number: u32,
    name: String,
    normal: Normal,
    group: AccountGroup,
    iscontra: bool,
    iscash: bool,
    report: Report
}

impl Account {
    fn new(number: u32, name: &str, normal: Normal, group: AccountGroup, report: Report) -> Self {
        Self { 
            number, 
            name: name.to_string(),
            normal,
            group,
            report,
            ..Default::default()
        }
    }
}

pub enum TransType { DEBIT = 1, CREDIT = -1 }

pub struct Transaction {
    number: u32,
    date: String,
    account: u32,
    description: String,
    dc: TransType,
    amount: u128
}

pub struct COA(HashMap<u32, Account>);

impl COA {
    pub fn new() -> Self {
        Self(HashMap::<u32, Account>::new())
    }

    pub fn print_all(&self) {
        for (key, val) in self.0.iter() {
            println!("{}\t| {} ({:?})", key, val.name, val.group)
        }
    }

    pub fn add(&mut self, acc: Account) {
        self.0.insert(acc.number, acc);
    }

    pub fn try_add(&mut self, acc: Account) -> Result<(), String> {
        match self.0.contains_key(&acc.number) {
            true => {
                let msg = format!("Account with key {} already exist!", acc.number);
                Err(msg)
            } 
            false => {
                self.0.insert(acc.number, acc);
                Ok(())
            }
        }
    }

    pub fn get_mut(&mut self, number: u32) -> Option<&mut Account> {
        self.0.get_mut(&number)
    }

    pub fn get_ref(&self, number: u32) -> Option<&Account> {
        self.0.get(&number)
    }
}


pub fn test_coa() {
    let mut coa = COA::new();

    let kas = Account::new(11001, "Kas", Normal::DEBIT, AccountGroup::ASSET, Report::BS);
    let bank = Account::new(11002, "Bank", Normal::DEBIT, AccountGroup::ASSET, Report::BS);
    let modal = Account::new(31001, "Modal", Normal::CREDIT, AccountGroup::EQUITY, Report::BS);
    let hutang = Account::new(25001, "Hutang", Normal::CREDIT, AccountGroup::LIABILITY, Report::BS);
    let pendapatan = Account::new(31001, "Pendapatan", Normal::CREDIT, AccountGroup::REVENUE, Report::IS);
    let beban = Account::new(41001, "Beban", Normal::CREDIT, AccountGroup::EXPENSE, Report::IS);
    let laba = Account::new(81001, "Laba", Normal::CREDIT, AccountGroup::INCOME, Report::IS);

    coa.add(kas);
    coa.add(bank);
    coa.add(modal);
    coa.add(hutang);
    coa.add(pendapatan);
    coa.add(beban);
    coa.add(laba);

    coa.print_all();

    if let Some(b) = coa.get_mut(11002) {
        b.name = "BNI".to_string();
    }

    coa.print_all();
}