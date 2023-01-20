use genio::{Read, Write};
use rand::{CryptoRng, RngCore};
use ssb_crypto::{Keypair, NetworkKey};
use ssb_crypto::ephemeral::generate_ephemeral_keypair_with_rng;
use ssb_handshake::HandshakeError;
use ssb_handshake::sync::server_side;


/// Run the server process.  Returns `Ok(())` if the handshake succeeds, `Err(_)` otherwise.
pub fn run<T, R>(
    rng: &mut R,
    channel: T,
) -> Result<(), HandshakeError<T::ReadError>>
where
    T: Read,
    T: Write<WriteError = T::ReadError, FlushError = T::ReadError>,
    R: CryptoRng + RngCore,
{
    let net_key = NetworkKey::SSB_MAIN_NET;
    cc_trace!("generating keypair");
    let keypair = Keypair::generate_with_rng(rng);
    cc_trace!("generating ephemeral keypair");
    let (eph_pk, eph_sk) = generate_ephemeral_keypair_with_rng(rng);

    cc_trace!("starting server_side");
    server_side(channel, &net_key, &keypair, (eph_pk, eph_sk))?;
    Ok(())
}
