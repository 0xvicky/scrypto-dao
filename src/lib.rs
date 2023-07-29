use scrypto::prelude::*;

blueprint! {
    struct TokenSale{
      vicky_vault:Vault,
      xrd_vault:Vault
    }

    impl TokenSale{
        pub fn new()-> ComponentAddress{
          let bucket:Bucket = ResourceBuilder::new_fungible()
          .metadata("name", "Vicky Token")
          .metadata("symbol","V-X")
          .initial_supply(100)

        Self{
vicky_vault:Vault  = Vault::with_bucket(bucket)
        }
      .instantiate()
      .globalize()
        }
    }
}
