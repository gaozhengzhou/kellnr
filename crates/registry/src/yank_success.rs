use json_payload::json_payload;

#[json_payload]
pub struct YankSuccess {
    ok: bool,
}

impl YankSuccess {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for YankSuccess {
    fn default() -> Self {
        Self { ok: true }
    }
}
