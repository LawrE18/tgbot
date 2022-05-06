pub trait CryptoProvider {
    type Transaction;
    type Signature;
    type Error;
    fn generate_keypairs(self) -> Result<Vec<u8>, Self::Error>;
    fn get_public(self) -> Result<Vec<u8>, Self::Error>;
    fn sign_transaction(
        self,
        transaction: Self::Transaction,
    ) -> Result<Self::Signature, Self::Error>;
}