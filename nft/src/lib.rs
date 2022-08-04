
#[derive(NonFungible)]
struct Picture {
    name: String,
    hash: Hash,
    url: String,
}

#[template]
mod nft {
    struct PictureSeller {
        pictures: Vault<Pictures>,
        price: ThaumAmount,
        earnings: Vault<Thaum>,
    }

    impl PictureSeller {
        pub fn new(&mut self, price: ThaumAmount) -> (Bucket<Picture>, Bucket<Thaum>) {
            let pictures = ResourceBuilder::new<Picture>().
                .metadata("name", "Famous paintings")
                .initial_supply(vec![
                    Picture::new(NonFungibleId.random(), "Mona Lisa", Hash::from_hex("0x...."),"https://...."),
                    Picture::new(NonFungibleId.random(), "The Starry Night", Hash::from_hex("0x...."), "https://..."),
                    Picture::new(NonFungibleId.random(), "Guernice", Hash::from_hex("0x...."), "https://..."),
                ])
                .into_vault();

            PictureSeller {
                pictures,
                price,
                earnings: Vault::new<Thaum>();
            }
        }

        pub fn buy(&mut self, mut payment: Bucket<Thaum>) -> (Bucket<Picture>, Bucket<Thaum>) {
            self.earnings.put(payments.take(self.price));

            let picture = self.pictures.take(1);

            (picture, payments)
        }
    }
}
