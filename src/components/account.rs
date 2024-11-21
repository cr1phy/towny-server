use entity::account;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct AccountSettings {
    enable_notifications: bool,
}

impl Default for AccountSettings {
    fn default() -> Self {
        Self {
            enable_notifications: true,
        }
    }
}

#[derive(Debug, Default)]
pub struct Account {
    id: Uuid,
    username: String,
    email: String,
    password: Vec<u8>,
    money: f64,
    gems: i64,
    settings: AccountSettings,
}

impl From<account::Model> for Account {
    fn from(value: account::Model) -> Self {
        Self {
            id: value.id,
            username: value.username,
            email: value.email,
            password: value.password,
            money: value.money,
            gems: value.gems,
            settings: serde_json::from_value(value.settings.to_owned()).unwrap(),
        }
    }
}

impl Account {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }

    pub fn password(&self) -> &[u8] {
        &self.password
    }

    pub fn set_password(&mut self, password: Vec<u8>) {
        self.password = password;
    }

    pub fn money(&self) -> f64 {
        self.money
    }

    pub fn set_money(&mut self, money: f64) {
        self.money = money;
    }

    pub fn gems(&self) -> i64 {
        self.gems
    }

    pub fn set_gems(&mut self, gems: i64) {
        self.gems = gems;
    }

    pub fn settings(&self) -> &AccountSettings {
        &self.settings
    }

    pub fn set_settings(&mut self, settings: AccountSettings) {
        self.settings = settings;
    }
}
