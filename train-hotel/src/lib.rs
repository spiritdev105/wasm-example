#[template]
mod train_hotel {

    pub struct TrainAndHotelApp {
        // What is actually stored is the CommponentId
        train_ticket_issuer: TrainTicketIssuer,
        hotel_room_issuer: HotelRoomIssuer,
        earnings: Thaum,
        fee: ThaumAmount,
    }

    impl TrainAndHotelApp {
        // the actual instruction call will need to pass the ComponentId for each of the issuers
        pub fn new(fee: ThaumAmount, train_ticket_issuer: TrainTicketIssuer, hotel_room_issuer: HotelRoomIssuer) -> (Self, Bucket<Badge>) {

        }

        pub fn buy(&mut self, mut payment: Bucket<Thaum>) -> (Bucket<TrainTicket>, Bucket<HotelReservation>, Bucket<Thaum>) {
            // if any of the calls fail, the whole transaction fails, atomically
            self.earnings.put(payment.take(self.fee));
            let (train_ticket, _) = self.train_ticket_issuer.buy_ticket(payment);
            let (hotel_reservation, _) = self.hotel_room_issuer.make_reservation(payment);

            // return the tickets and the change
            (train_ticket, hotel_reservation, payment)
        }
    }
    
}

#[template]
mod train {
    pub struct TrainTicketIssuer {
        tickets: Vault<TrainTicket>,
        price: ThaumAmount,
        earnings: Vault<Thaum>,
    }

    impl TrainTicketIssuer {
        pub fn new(price: ThaumAmount) -> (Self, Bucket<Badge>) {
            // TODO
        }

        pub fn buy_ticket(&mut self, payment: Bucket<Thaum>) -> (Bucket<TrainTicket>, Bucket<Thaum>) {
            let ticket = self.tickets.take(1);
            self.earnings.put(payment.take(self.price));

            (ticket, payment)
        }
    }
}

#[template]
mod hotel {
    pub struct HotelRoomIssuer {
        reservations: Vault<HotelReservation>,
        price: ThaumAmount,
        earnings: Vault<Thaum>,
    }

    impl HotelRoomIssuer {
        pub fn new(price: ThaumAmount) -> (Self, Bucket<Badge>) {
           // TODO
        }

        pub fn make_reservation(&mut self, mut payment: Bucket<Thaum>) -> (Bucket<HotelReservation>, Bucket<Thaum>) {
            let reservation = self.reservations.take(1);
            self.earnings.put(payment.take(self.price));

            (reservation, payment)
        }
    }
}
