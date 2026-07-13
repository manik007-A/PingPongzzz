#[derive(Debug, Clone)]
pub struct IdentityModel {
    pub id: i32,
    pub hostname: String,
    pub nickname: String,
    pub fingerprint: String,
}