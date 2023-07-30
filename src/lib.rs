use scrypto::prelude::*;

#[blueprint]
mod token_sale {
    struct TokenSale {
        // Define what resources and data will be managed by Hello components
        vicky_vault: Vault,
        xrd_vault: Vault,
        price_per_token: Decimal,
    }

    impl TokenSale {
        // Implement the functions and methods which will manage those resources and data
        // This is a function, and can be called directly on the blueprint once deployed
        pub fn new(price_per_token: Decimal) -> (Global<TokenSale>, Bucket) {
            // Create a new token called "HelloToken," with a fixed supply of 1000, and put that supply into a bucket
            let my_bucket: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .metadata(metadata!(
                    init {
                        "name" => "Vicky Token", locked;
                        "symbol" => "V-X", locked;
                    }
                ))
                .mint_initial_supply(1000);

            let seller_badge: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .metadata(metadata!(
                  init{
                    "name"=>"Seller Badge",locked;
                    "symbol"=>"SELLER",locked;
                    "description"=>"One who can withdraw XRD and can price_per_token",locked;
                  }
                ))
                .mint_initial_supply(1);

            //Giving access rules to the component
            let access_rules: AccessRules = AccessRules::new().method(
                "withdraw_funds",
                rule!(require(seller_badge.resource_address())),
            );
            // Instantiate a Hello component, populating its vault with our supply of 1000 HelloToken
            let component = Self {
                vicky_vault: Vault::with_bucket(my_bucket),
                xrd_vault: Vault::new(RADIX_TOKEN), //To instantiate radix token
                price_per_token: price_per_token,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize();

            (component, seller_badge)
        }

        // This is a method, because it needs a reference to self.  Methods can only be called on components
        pub fn buy(&mut self, funds: Bucket) -> Bucket {
            info!(
                "My balance is: {} vicky token. Now giving away some tokens!",
                self.vicky_vault.amount()
            );

            let purchase_amount = funds.amount() / self.price_per_token;
            self.xrd_vault.put(funds);
            self.vicky_vault.take(purchase_amount)
        }

        pub fn withdraw(&mut self, amount: Decimal) -> Bucket {
            info!("Withdrawing Funds...");
            self.xrd_vault.take(amount)
        }

        pub fn change_price(&mut self, new_price: Decimal) {
            self.price_per_token = new_price
        }
    }
}
