
#[derive(Fungible)]
struct LiquidityToken {}

#[template]
mod dex {
    // The generics may not need to be "real" valid Rust generics, as if we use a procedural macro,
    // we could create our own generic rules and compile it to a dynamic type
    struct Dex<A, B> {
        a_tokens: Vault<A>,
        b_tokens: Vault<B>,
        fee: Amount,
        lp_resource_address: ResourceAddress<LiquidityToken>,
        lp_mint_badge: Vault<Badge>,
        lp_per_asset_ratio: Amount,
    }

    impl<A, B> Dex<A, B> where A: Fungible, B: Fungible
    {
        pub fn new(
            a_tokens: Bucket<A>,
            b_tokens: Bucket<B>,
            fee: ThaumAmount,
            lp_initial_supply: u64,
            lp_symbol: String,
            lp_name: String,     
        ) -> Self {
            // instantiate an initial supply of lp_tokens. Make them mintable and burnable only from this component
            // set lp_per_asset_ratio = lp_initial_supply / (a_tokens.amount() * b_tokens.amount());
        }

        pub fn add_liquidity(&mut self, mut a_tokens: Bucket<A>, mut b_tokens: Bucket<B>) -> Bucket<LiquidityToken> {
            // add the tokens to their respective pools
            // use the ratio to calculate how may liquitity tokens we need to mint and return as a result
            // recalculate the ratio
            // a check if the pool is empty may be needed as an special edge case
        }

        pub fn remove_liquidity(&mut self, lp_tokens: Bucket<LiquidityToken>) -> (Bucket<A>, Bucket<B>) {
            // calculate the amount of A and B tokens to withdraw based on the amount of lp_tokens
            // burn the received lp_tokens
        }

        // fairly complicated to do with generics
        // in most cases the user will only input one of the buckets and leave the other empty
        // another possiblity is to create our own generic rules (not Rust-compatible) and allow a dynamic Bucket as parameter
        pub fn swap(&mut self, input_a: Bucket<A>, input_b: Bucket<B>) -> (Bucket<A>, Bucket<B>) {
            // apply the swap fee
            // get the correct amount of B tokens from the A tokens provided
            // get the correct amount of A tokens from the B tokens provided
            // update the ratio
        }

        pub fn get_token_addresses(&self) -> (ResourceAddress<A>, ResourceAddress<B>) {
            (
                self.a_pool.resource_address(),
                self.b_pool.resource_address(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    
    #[template_stub(module="Dex")]
    struct Dex;

    #[template_stub(path="test/erc20", module="Koin")]
    struct Koin;

    #[template_test]
    fn swap_tokens() {
        // initialize a user account with funds
        // The token "A" will be Thaums
        let account = Account::new();
        account.add_fungible(Thaum, Amount(1_000));
        let thaum_bucket = account.take(100);

        // intitialize the token "B"
        let koin = Koin::initial_mint(Amount(1_000_000));
        let koin_bucket = koin.take_koins(Amount(100)).unwrap();
        
        // initialize the component
        // A: Thaum - B: Koin
        let fee = Amount(1);
        let mut dex = Dex::new(
            thaum_bucket,
            koin_bucket,
            fee: ThaumAmount(1),
            lp_initial_supply: 1000,
            lp_symbol: "TK",
            lp_name: "ThaumKoin",
        );

        // let's add Thaum liquidity
        let thaum_liquidity = account.take(50);
        let lp_tokens: Bucket<LiquidityToken> = dex.add_liquidity(thaum_liquidity, Bucket::new()).unwrap();

        // let's remove liquidity (we get both tokens)
        let (more_thaums, more_koins) = dex.remove_liquidity(lp_tokens).unwrap();
        account.add_fungible(more_thaums);
        account.add_fungible(more_koins);

        // let's swap some Koins for Thaums
        let some_koins = koin.take_koins(Amount(100));
        let (more_thaums, more_koins) = dex.swap(Bucket::new(), some_koins).unwrap();
        account.add_fungible(more_thaums);
        account.add_fungible(more_koins);

        // TODO: assert the expecting exchange ratio
    }
}
