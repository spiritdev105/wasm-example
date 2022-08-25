pub enum Emoji {
    Smile,
    Sweat,
    Laugh,
    Wink,
}

// the macro will immplement a "NonFungible" trait and a constructor "new(component_id, emojis)"
#[derive(NonFungible)]
pub struct EmojiId {
    pub emojis: Vec<Emoji>,
}

const MAX_EMOJI_ID_LEN: usize = 10;

#[template]
mod emoji_id {
    use crate::MAX_EMOJI_ID_LEN;

    struct EmojiIdMinter {
        owner_only: AccessRule,
        emoji_id_resource_address: ResourceAddress<EmojiId>,
        emoji_id_resource_badge: Vault<Badge>,
        earnings: Vault<Thaum>,
        price: Amount,
    }

    impl EmojiIdMinter {
        pub fn new(price: Amount) -> (Self, Bucket<Badge>) {
            let emoji_id_resource_badge = ResourceBuilder::new::<Badge>()
                .initial_supply(1);

            let emoji_id_resource_address = ResourceBuilder::new::<EmojiId>()
                // only the component will be able to mint, and the restriction cannot change 
                .mintable(AccessRule::require(emoji_id_resource_badge), LOCKED)
                .no_initial_supply();
            
            // set up the access rule of the owner
            let owner_badge = ResourceBuilder::new::<Badge>()
                .initial_supply(1);
            let owner_only = AccessRule::require(owner_badge);  

            let minter = EmojiIdMinter {
                owner_only,
                emoji_id_resource_address,
                emoji_id_resource_badge: Vault::with_bucket(emoji_id_resource_badge),
                earnings: Vault::new::<Thaum>(),
                price,
            };           

            (minter, owner_badge)
        }

        pub fn mint(&mut self, emojis: Vec<Emoji>, payment: Bucket<Thaum>) -> (Bucket<EmojiId>, Bucket<Thaum>) {
            assert!(
                !emojis.empty() || emojis.len() > MAX_EMOJI_ID_LEN,
                "Invalid Emoji ID lenght"
            );

            // process the payment
            // OLD version: self.earnings.put(payment.take(self.price));
            let (cost, change) = payment.split(self.price); // buckets are consumed
            self.earnings.put(cost);

            // the id of the resource is directly calculated from the emojis themselves, to ensure uniqueness of the emoji sequence
            let non_fungible_id = NonFungibleId::from_value(emojis);

            // mint a new emoji id
            // "mint_non_fungible" should panic if the id already exists
            let emoji_id = ResourceManager::borrow::<EmojiId>(self.emoji_id_resource_address)
                .use_badge(self.emoji_id_resource_badge)
                .mint_non_fungible(&non_fungible_id, &emojis);

            // return the newly minted emojiId and the payment change
            (emoji_id, change)
        }

        #[access_rule(owner_only)]
        pub fn withdraw_earnings(&mut self) -> Bucket<Thaum> {
            self.earnings.take_all()
        }
    }
}


#[cfg(test)]
mod tests {
    
    #[template_stub(module="EmojiIdMinter")]
    struct EmojiIdMinter;

    #[test]
    fn mint_and_withdraw() {
        // initialize the contract owner account
        let mut owner_account = Account::new();
        owner_account.add_badge(owner_badge);

        // initialize the component
        let price = Amount(1_000);
        let (mut emoji_id_minter, owner_badge) = EmojiIdMinter::new(price); 
        
        // initialize a user account with enough funds
        let mut user_account = Account::new();
        user_account.add_fungible(Thaum, price);

        // mint a new emoji id
        let emojis = vec![Emoji::Smile, Emoji::Wink];
        let payment: Bucket<Thaum> = user_account.take_fungible(Thaum, price);
        let (emoji_id, _) = emoji_id_minter.mint(emojis, paymet).unwrap();

        // store our brand new emoji_id in our account
        println!("Succesfully bought '{}'", emoji_id);
        user_account.add_non_fungible(emoji_id);

        // as owners, and we want to withdraw earnings
        let earnings: Bucket<Thaum> = emoji_id_minter
            .with_badge(owner_badge)    // derived by the "template_stub" macro
            .withdraw_earnings()
            .unwrap();
        owner_account.add_fungible(Thaum, earnings);
    }
}
