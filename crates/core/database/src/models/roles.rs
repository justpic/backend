#[derive(Debug, Clone)]
pub enum Role {
    Regular,
    Moderator,
    Admin,
}

impl From<String> for Role {
    fn from(value: String) -> Self {
        {
            let raw: &str = &value;
            match raw {
                "moderator" => Role::Moderator,
                "admin" => Role::Admin,
                _ => Role::Regular,
            }
        }
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Role {
    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            Role::Regular => "regular",
            Role::Moderator => "moderator",
            Role::Admin => "admin",
        }
    }
}
