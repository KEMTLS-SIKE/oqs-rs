use oqs::*;
use std::time::Instant;
fn main() -> Result<()> {
    oqs::init();
    let kemalg = kem::Kem::new(kem::Algorithm::SikeP434Compressed).unwrap();

    // A -> B: kem_pk
    let start = Instant::now();
    kemalg.init()?;
    let elapsed = start.elapsed();
    println!("Init Millis: {} ms", elapsed.as_millis());

    let start = Instant::now();
    for _ in  1..10 {
        let (kem_pk, kem_sk) = kemalg.keypair_async()?;
        let (kem_ct, b_kem_ss) = kemalg.encapsulate(&kem_pk)?;
        let a_kem_ss = kemalg.decapsulate(&kem_sk, &kem_ct)?;
        assert_eq!(a_kem_ss, b_kem_ss);
    }
    let elapsed = start.elapsed();
    println!("Normal Encapsulate millis: {} ms", elapsed.as_millis());

    // A -> B: kem_pk
    let start = Instant::now();
    for _ in  1..10 {
        let (kem_pk, kem_sk) = kemalg.keypair_async()?;
        let (kem_ct, b_kem_ss) = kemalg.async_encapsulate(&kem_pk)?;
        let a_kem_ss = kemalg.decapsulate(&kem_sk, &kem_ct)?;
        assert_eq!(a_kem_ss, b_kem_ss);
    }
    let elapsed = start.elapsed();
    println!("Async Encapsulate millis: {} ms", elapsed.as_millis());

    oqs::sike_deinit();

    Ok(())
}
