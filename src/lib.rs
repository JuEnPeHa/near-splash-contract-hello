use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        let this = Self { owner_id };
        this
    }

    pub fn spanish_near(&self, name: String) -> String {
        "Hola ".to_string() + &name + "!"
    }
    pub fn english_near(&self, name: String) -> String {
        "Hello ".to_string() + &name + "!"
    }
    pub fn bonjour_near(&self, name: String) -> String {
        "Bonjour ".to_string() + &name + "!"
    }
    pub fn russian_near(&self, name: String) -> String {
        "Привет ".to_string() + &name + "!"
    }
    pub fn ukrainian_near(&self, name: String) -> String {
        "Привіт ".to_string() + &name + "!"
    }
    pub fn italian_near(&self, name: String) -> String {
        "Ciao ".to_string() + &name + "!"
    }
    pub fn turkish_near(&self, name: String) -> String {
        "Merhaba ".to_string() + &name + "!"
    }
    pub fn chinise_near(&self, name: String) -> String {
        "你好 ".to_string() + &name + "!"
    }
    pub fn portuguese_near(&self, name: String) -> String {
        "Olá ".to_string() + &name + "!"
    }
}
