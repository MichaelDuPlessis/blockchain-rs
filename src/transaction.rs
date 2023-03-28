pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: u64) -> Self {
        Self { from, to, amount }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        [
            self.from.as_bytes(),
            self.to.as_bytes(),
            &self.amount.to_be_bytes(),
        ]
        .into_iter()
        .flatten()
        .map(|b| *b)
        .collect::<Vec<u8>>()
    }

    pub fn from(&self) -> &str {
        &self.from
    }

    pub fn to(&self) -> &str {
        &self.to
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }
}
