use oqs::*;
use std::time::Instant;
fn main() -> Result<()> {
    oqs::init();
    let kemalg = kem::Kem::new(kem::Algorithm::SikeP434Compressed).unwrap();

    // A -> B: kem_pk
    let start = Instant::now();
    let _res = kemalg.init().ok()?;
    let elapsed = start.elapsed();
    println!("Millis: {} ms", elapsed.as_millis());

    let start = Instant::now();
    for _ in  1..10 {
        let (_kem_pk, _kem_sk) = kemalg.keypair_async()?;
        //let (kem_ct, b_kem_ss) = kemalg.encapsulate(&kem_pk)?;
        //let a_kem_ss = kemalg.decapsulate(&kem_sk, &kem_ct)?;
        //assert_eq!(a_kem_ss, b_kem_ss);
    }
    let elapsed = start.elapsed();
    println!("Millis: {} ms", elapsed.as_millis());

    Ok(())
}
