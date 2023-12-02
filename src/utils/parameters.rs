pub struct Parameters {
    params: Vec<String>,
}

impl Default for Parameters {
    fn default() -> Self {
        Self::new()
    }
}

impl Parameters {
    pub fn new() -> Self {
        Parameters { params: Vec::new() }
    }

    pub fn format(&self) -> String {
        if self.params.is_empty() {
            String::new()
        } else {
            format!("?{}", self.params.join("&"))
        }
    }

    pub fn count(&mut self, amount: i32) {
        self.params.push(format!("count={}", amount));
    }

    pub fn cursor(&mut self, cursor: &str) {
        self.params.push(format!("cursor={}", cursor));
    }

    pub fn policy(&mut self, policy: &str) {
        self.params.push(format!("policy={}", policy));
    }

    pub fn epoch_no(&mut self, epoch_no: i64) {
        self.params.push(format!("epoch_no={}", epoch_no));
    }

    pub fn from(&mut self, from: i64) {
        self.params.push(format!("from={}", from));
    }

    pub fn to(&mut self, to: i64) {
        self.params.push(format!("to={}", to));
    }

    pub fn set_asc_order(&mut self) {
        self.params.push("order=asc".to_string());
    }

    pub fn set_desc_order(&mut self) {
        self.params.push("order=desc".to_string());
    }

    pub fn with_cbor(&mut self) {
        self.params.push("with_cbor=true".to_string());
    }

    pub fn resolve_datums(&mut self) {
        self.params.push("resolve_datums=true".to_string());
    }

    pub fn from_height(&mut self, height: i64) {
        self.params.push(format!("from_height={}", height));
    }
}
