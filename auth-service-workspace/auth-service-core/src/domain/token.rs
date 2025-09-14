#[derive(serde::Serialize, serde::Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct Token(String);
impl Token {
    pub fn new(token: String) -> Self {
        Token(token)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn take_string(self) -> String {
        self.0
    }
}
impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token(\"****\")")
    }
}
