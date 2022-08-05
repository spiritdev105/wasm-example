
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
