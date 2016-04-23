

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

struct MessageBin {
    // Message op code / message type.
    op:     u8,
    // Hardware address type, see ARP section in "Assigned Numbers" RFC; e.g., '1' = 10mb ethernet.
    htype:  u8,
    // Hardware address length (e.g. '6' for 10mb ethernet).
    hlen:   u8,
    // Client sets to zero, optionally used by relay agents when booting via a relay agent.
    hops:   u8,
    // Transaction ID, a random number chosen by the client, used by the client and server to
    // associate messages and responses between a client and a server.
    xid:    u32,
    // Filled in by client, seconds elapsed since client began address acquisition or renewal
    // process.
    secs:   u16,
    flags:  u16,
    // Client IP address; only filled in if client is in BOUND, RENEW or REBINDING state and can
    // respond to ARP requests.
    ciaddr: u32,
    // 'your' (client) IP address.
    yiaddr: u32,
    // IP address of next server to use in bootstrap; returned in DHCPOFFER, DHCPACK by server.
    siaddr: u32,
    // Relay agent IP address, used in booting via a relay agent.
    giaddr: u32,
    // Client hardware address.
    chaddr: [u32; 4],
    // Optional server host name, null terminated string.
    sname: [u32; 16],
    // Boot file name, null terminated string; "generic" name or null in DHCPDISCOVER, fully
    // qualified directory-path name in DHCPOFFER.
    file: [u32; 32],
    // Optional parameters field.
    /* The 'options' field is now variable length. A DHCP client must be prepared to receive DHCP
     * messages with an 'options' field of at least length 312 octets.  This requirement implies
     * that a DHCP client must be prepared to receive a message of up to 576 octets, the minimum IP
     * datagram size an IP host must be prepared to accept [3].  DHCP clients may negotiate the use
     * of larger DHCP messages through the 'maximum DHCP message size' option.  The options field
     * may be further extended into the 'file' and 'sname' fields.
     */
    options: Vec<()>
}

enum MsgOpField { BootRequest = 1, BootReply = 2}
struct MessageLogic {
    op: MsgOpField,

}


