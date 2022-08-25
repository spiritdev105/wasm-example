
#[derive(NonFungible)]
struct Picture {
    name: String,
    // maybe the hash should be automatically derived from "NonFungible"
    hash: Hash,
    url: String,
}

#[template]
mod nft {
    struct PictureSeller {
        pictures: Vault<Picture>,
        price: Amount,
        earnings: Vault<Thaum>,
    }

    impl PictureSeller {
        pub fn new(&mut self, price: Amount) -> PictureSeller {
            let pictures = ResourceBuilder::new::<Picture>()
                .metadata("name", "Famous paintings")
                .initial_supply(vec![
                    Picture::new(NonFungibleId.random(), "Mona Lisa", Hash::from_hex("0x...."), "https://...."),
                    Picture::new(NonFungibleId.random(), "The Starry Night", Hash::from_hex("0x...."), "https://..."),
                    Picture::new(NonFungibleId.random(), "Guernica", Hash::from_hex("0x...."), "https://..."),
                ])
                .into_vault();

            PictureSeller {
                pictures,
                price,
                earnings: Vault::new::<Thaum>(),
            }
        }

        pub fn buy(&mut self, payment: Bucket<Thaum>) -> (Bucket<Picture>, Bucket<Thaum>) {
            let (cost, change) = payment.split(self.cost);
            self.earnings.put(cost);

            let picture = self.pictures.take(1);

            (picture, change)
        }
    }
}

#[cfg(test)]
mod tests {
    
    #[template_stub(module="PictureSeller")]
    struct PictureSeller;

    #[template_test]
    fn it_should_let_buy_with_enough_funds() {
        // initialize the component
        let price = Amount(1_000);
        let mut picture_seller = PictureSeller::new(price);
        
        // initialize a user account with enough funds
        let mut account = Account::new();
        account.add_fungible(ThaumFaucet::take(price));

        // buy a picture
        let payment: Bucket<Thaum> = account.take_fungible(price);
        let (picture, _) = picture_seller.buy(payment).unwrap();

        // store our brand new picture in our account
        println!("Succesfully bought '{}'", picture.name);
        account.add_non_fungible(picture);
    }

    #[template_test]
    fn it_should_not_let_buy_with_insufficient_funds() {
        // initialize the component
        let price = Amount(1_000);
        let mut picture_seller = PictureSeller::new(price);

        // initialize a user account
        let mut account = Account::new();
        account.add_fungible(ThaumFaucet::take(price));

        // try to buy with an insufficient payment...
        let insufficient_amount = Amount(price - 1);
        let payment: Bucket<Thaum> = account.take_fungible(insufficient_amount);

        // ...we should get an error
        picture_seller.buy(payment).unwrap_err();
    }

    #[template_test]
    fn it_should_fail_with_no_pictures_left() {
        // initialize the component
        let price = Amount(1_000);
        let mut picture_seller = PictureSeller::new(price);

        // initialize a user account with enough funds to buy all pictures
        let mut account = Account::new();
        account.add_fungible(ThaumFaucet::take(price * 4));

        // buy all pictures
        for _ in 0..3 {
            let payment: Bucket<Thaum> = account.take_fungible(price);
            let (picture, _) = picture_seller.buy(payment).unwrap();
            account.add_non_fungible(picture);
        }

        // now there are no more pictures left, if we try to buy again we will get an error
        let payment: Bucket<Thaum> = account.take_fungible(price);
        picture_seller.buy(payment).unwrap_err();
    }
}
