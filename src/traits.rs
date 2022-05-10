pub trait CryptoProvider {
    type Transaction;
    type Signature;
    type Error;
    fn generate_keypairs(self) -> Result<String, Self::Error>;
    fn get_public(self) -> Result<String, Self::Error>;
    fn sign_transaction(self, transaction: Self::Transaction) -> Result<String, Self::Error>;
}
