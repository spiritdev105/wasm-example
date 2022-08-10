

// This is an example of an atomic composable template
// The user wants to buy a train ticket AND a hotel reservation atomically,
// the user will buy either BOTH at the same time or NONE
#[template]
mod train_hotel {

    pub struct TrainAndHotelApp {
        // What is actually stored is the CommponentId
        train_ticket_issuer: TrainTicketIssuer,
        hotel_room_issuer: HotelRoomIssuer,
        earnings: Vault<Thaum>,
        fee: Amount,
    }

    impl TrainAndHotelApp {
        // The actual instruction call will need to pass the ComponentId for each of the issuers, and ...
        // ... it gets automatically converted to the instance of the Component
        // Another option would be to explicitly use a "ContractAddress" type
        pub fn new(fee: Amount, train_ticket_issuer: TrainTicketIssuer, hotel_room_issuer: HotelRoomIssuer) -> (Self, Bucket<Badge>) {
            // instantiate the TrainAndHotelApp with the provided parameters
            // create and return an adming badge to withdraw earnings
        }

        pub fn buy(&mut self, payment: Bucket<Thaum>) -> (Bucket<TrainTicket>, Bucket<HotelReservation>, Bucket<Thaum>) {
            // if any of the calls fails, the whole transaction fails, atomically

            // apply the fee of the app
            let (fee_payment, change) = payment.split(self.fee);
            self.earnings.put(fee_payment);

            // buy the train ticket
            let (train_ticket, change) = self.train_ticket_issuer.buy_ticket(change);

            // make the hotel reservation
            let (hotel_reservation, change) = self.hotel_room_issuer.make_reservation(change);

            // return the tickets and the change
            (train_ticket, hotel_reservation, change)
        }
    }
    
}

#[template]
mod train {
    pub struct TrainTicketIssuer {
        tickets: Vault<TrainTicket>,
        price: Amount,
        earnings: Vault<Thaum>,
    }

    impl TrainTicketIssuer {
        pub fn new(price: Amount) -> (Self, Bucket<Badge>) {
            // TODO
        }

        pub fn buy_ticket(&mut self, payment: Bucket<Thaum>) -> (Bucket<TrainTicket>, Bucket<Thaum>) {
            // get the ticket
            // this is a very simple example, in a real scenario the user would want to specifiy destination, time, etc
            let ticket = self.tickets.take(1);

            // process the payment
            let (cost, change) = payment.split(self.price);
            self.earnings.put(cost);       

            (ticket, change)
        }

        // in a real app, there should be an "admin" protected method to add/remove available tickets
        // ... and a open one to search for tickets
    }
}

#[template]
mod hotel {
    pub struct HotelRoomIssuer {
        reservations: Vault<HotelReservation>,
        price: Amount,
        earnings: Vault<Thaum>,
    }

    impl HotelRoomIssuer {
        pub fn new(price: Amount) -> (Self, Bucket<Badge>) {
           // TODO
        }

        pub fn make_reservation(&mut self, mut payment: Bucket<Thaum>) -> (Bucket<HotelReservation>, Bucket<Thaum>) {
            // make a room reservation
            // this is a very simple example, in a real scenario the user would want to specifiy the room type, check-in, etc
            let reservation = self.reservations.take(1);

            // process the payment
            let (cost, change) = payment.split(self.price);
            self.earnings.put(cost);       

            (reservation, change)
        }

        // in a real app, there should be an "admin" protected method to add/remove available rooms
        // ... and a open one to search for rooms
    }
}
