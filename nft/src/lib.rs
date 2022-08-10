
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
        pictures: Vault<Pictures>,
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
                    Picture::new(NonFungibleId.random(), "Guernice", Hash::from_hex("0x...."), "https://..."),
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
