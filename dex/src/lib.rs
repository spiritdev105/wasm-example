
#[derive(Fungible)]
struct LiquidityToken {}

#[template]
mod dex {
    struct Dex<A, B> {
        a_tokens: Vault<A>,
        b_tokens: Vault<B>,
        fee: ThaumAmount,
        lp_resource_address: ResourceAddress<LiquidityToken>,
        lp_mint_badge: Vault<Badge>,
        lp_per_asset_ratio: u64,
    }

    impl<A, B> Dex<A, B> where A: Fungible, B: Fungible
    {
        pub new(
            a_tokens: Bucket<A>,
            b_tokens: Bucket<B>,
            fee: ThaumAmount,
            lp_initial_supply: u64,
            lp_symbol: String,
            lp_name: String,     
        ) -> Self {
            
        }

        pub fn add_liquidity(&mut self, mut a_tokens: Bucket<A>, mut b_tokens: Bucket<B>) -> Bucket<LiquidityToken> {
            
        }

        pub fn remove_liquidity(&mut self, lp_tokens: Bucket<LiquidityToken>) -> (Bucket<A>, Bucket<B>) {
            
        }

        // fairly comlicated to do with generics
        // user can perfectly input some "A" tokens but no "B" tokens or viceversa
        pub fn swap(&mut self, input_a: Bucket<A>, input_b: Bucket<B>) -> (Bucket<A>, Bucket<B>) {

        }

        pub fn get_token_addresses(&self) -> (ResourceAddress<A>, ResourceAddress<B>) {
            (
                self.a_pool.resource_address(),
                self.b_pool.resource_address(),
            )
        }
    }
}
