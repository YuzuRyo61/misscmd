#[derive(Deserialize)]
pub struct MiAuthResponse {
    pub ok: bool,
    pub token: Option<String>
}
