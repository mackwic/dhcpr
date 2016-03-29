

pub enum S2CMessages {
    Offer,
    Ack,
    Nack,
}

pub enum C2SMessage {
    Discover,
    Request,
    Decline,
    Release,
    Inform
}
