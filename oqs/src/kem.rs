//! KEM API
//!
//! See [`Kem`] for the main functionality.
//! [`Algorithm`] lists the available algorithms.
use alloc::vec::Vec;

use core::ptr::NonNull;

#[cfg(not(feature = "std"))]
use cstr_core::CStr;
#[cfg(feature = "std")]
use std::ffi::CStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::ffi::kem as ffi;
use crate::newtype_buffer;
use crate::*;

newtype_buffer!(PublicKey, PublicKeyRef);
newtype_buffer!(SecretKey, SecretKeyRef);
newtype_buffer!(Ciphertext, CiphertextRef);
newtype_buffer!(SharedSecret, SharedSecretRef);
newtype_buffer!(EphemeralSecret, EphemeralSecretRef);

macro_rules! implement_kems {
    { $(($feat: literal) $kem: ident: $oqs_id: ident),* $(,)? } => (

        /// Supported algorithms by OQS
        ///
        /// Note that this doesn't mean that they'll be available.
        ///
        /// Optional support for `serde` if that feature is enabled.
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[allow(missing_docs)]
        pub enum Algorithm {
            $(
                $kem,
            )*
        }

        fn algorithm_to_id(algorithm: Algorithm) -> *const libc::c_char {
            let id: &[u8] = match algorithm {
                $(
                    Algorithm::$kem => &ffi::$oqs_id[..],
                )*
            };
            id as *const _ as *const i8
        }

        $(
            #[cfg(test)]
            #[allow(non_snake_case)]
            mod $kem {
                use super::*;

                #[test]
                #[cfg(feature = $feat)]
                fn test_encaps_decaps() -> Result<()> {
                    crate::init();

                    let alg = Algorithm::$kem;
                    let kem = Kem::new(alg)?;
                    let (pk, sk) = kem.keypair()?;
                    let (ct, ss1) = kem.encapsulate(&pk)?;
                    let ss2 = kem.decapsulate(&sk, &ct)?;
                    assert_eq!(ss1, ss2, "shared secret not equal!");
                    Ok(())
                }

                #[test]
                fn test_enabled() {
                    crate::init();
                    if cfg!(feature = $feat) {
                        assert!(Algorithm::$kem.is_enabled());
                    } else {
                        assert!(!Algorithm::$kem.is_enabled())
                    }
                }

                #[test]
                fn test_name() {
                    let algo = Algorithm::$kem;
                    // Just make sure the name impl does not panic or crash.
                    let name = algo.name();
                    #[cfg(feature = "std")]
                    assert_eq!(name, algo.to_string());
                    // ... And actually contains something.
                    assert!(!name.is_empty());
                }

                #[test]
                fn test_get_algorithm_back() {
                    let algorithm = Algorithm::$kem;
                    if algorithm.is_enabled() {
                        let kem = Kem::new(algorithm).unwrap();
                        assert_eq!(algorithm, kem.algorithm());
                    }
                }

                #[test]
                fn test_version() {
                    if let Ok(kem) = Kem::new(Algorithm::$kem) {
                        // Just make sure the version can be called without panic
                        let version = kem.version();
                        // ... And actually contains something.
                        assert!(!version.is_empty());
                    }
                }
            }
        )*
    )
}

implement_kems! {
    ("bike") BikeL1: OQS_KEM_alg_bike_l1,
    ("bike") BikeL3: OQS_KEM_alg_bike_l3,
    ("classic_mceliece") ClassicMcEliece348864: OQS_KEM_alg_classic_mceliece_348864,
    ("classic_mceliece") ClassicMcEliece348864f: OQS_KEM_alg_classic_mceliece_348864f,
    ("classic_mceliece") ClassicMcEliece460896: OQS_KEM_alg_classic_mceliece_460896,
    ("classic_mceliece") ClassicMcEliece460896f: OQS_KEM_alg_classic_mceliece_460896f,
    ("classic_mceliece") ClassicMcEliece6688128: OQS_KEM_alg_classic_mceliece_6688128,
    ("classic_mceliece") ClassicMcEliece6688128f: OQS_KEM_alg_classic_mceliece_6688128f,
    ("classic_mceliece") ClassicMcEliece6960119: OQS_KEM_alg_classic_mceliece_6960119,
    ("classic_mceliece") ClassicMcEliece6960119f: OQS_KEM_alg_classic_mceliece_6960119f,
    ("classic_mceliece") ClassicMcEliece8192128: OQS_KEM_alg_classic_mceliece_8192128,
    ("classic_mceliece") ClassicMcEliece8192128f: OQS_KEM_alg_classic_mceliece_8192128f,
    ("hqc") Hqc128: OQS_KEM_alg_hqc_128,
    ("hqc") Hqc192: OQS_KEM_alg_hqc_192,
    ("hqc") Hqc256: OQS_KEM_alg_hqc_256,
    ("kyber") Kyber512: OQS_KEM_alg_kyber_512,
    ("kyber") Kyber768: OQS_KEM_alg_kyber_768,
    ("kyber") Kyber1024: OQS_KEM_alg_kyber_1024,
    ("kyber") Kyber512_90s: OQS_KEM_alg_kyber_512_90s,
    ("kyber") Kyber768_90s: OQS_KEM_alg_kyber_768_90s,
    ("kyber") Kyber1024_90s: OQS_KEM_alg_kyber_1024_90s,
    ("ntru") NtruHps2048509: OQS_KEM_alg_ntru_hps2048509,
    ("ntru") NtruHps2048677: OQS_KEM_alg_ntru_hps2048677,
    ("ntru") NtruHps4096821: OQS_KEM_alg_ntru_hps4096821,
    ("ntru") NtruHps40961229: OQS_KEM_alg_ntru_hps40961229,
    ("ntru") NtruHrss701: OQS_KEM_alg_ntru_hrss701,
    ("ntru") NtruHrss1373: OQS_KEM_alg_ntru_hrss1373,
    ("ntruprime") NtruPrimeNtrulpr653: OQS_KEM_alg_ntruprime_ntrulpr653,
    ("ntruprime") NtruPrimeNtrulpr761: OQS_KEM_alg_ntruprime_ntrulpr761,
    ("ntruprime") NtruPrimeNtrulpr857: OQS_KEM_alg_ntruprime_ntrulpr857,
    ("ntruprime") NtruPrimeNtrulpr1277: OQS_KEM_alg_ntruprime_ntrulpr1277,
    ("ntruprime") NtruPrimeSntrup653: OQS_KEM_alg_ntruprime_sntrup653,
    ("ntruprime") NtruPrimeSntrup761: OQS_KEM_alg_ntruprime_sntrup761,
    ("ntruprime") NtruPrimeSntrup857: OQS_KEM_alg_ntruprime_sntrup857,
    ("ntruprime") NtruPrimeSntrup1277: OQS_KEM_alg_ntruprime_sntrup1277,
    ("saber") Lightsaber: OQS_KEM_alg_saber_lightsaber,
    ("saber") Saber: OQS_KEM_alg_saber_saber,
    ("saber") Firesaber: OQS_KEM_alg_saber_firesaber,
    ("frodokem") FrodoKem640Aes: OQS_KEM_alg_frodokem_640_aes,
    ("frodokem") FrodoKem640Shake: OQS_KEM_alg_frodokem_640_shake,
    ("frodokem") FrodoKem976Aes: OQS_KEM_alg_frodokem_976_aes,
    ("frodokem") FrodoKem976Shake: OQS_KEM_alg_frodokem_976_shake,
    ("frodokem") FrodoKem1344Aes: OQS_KEM_alg_frodokem_1344_aes,
    ("frodokem") FrodoKem1344Shake: OQS_KEM_alg_frodokem_1344_shake,
    ("sidh") SidhP434: OQS_KEM_alg_sidh_p434,
    ("sidh") SidhP503: OQS_KEM_alg_sidh_p503,
    ("sidh") SidhP610: OQS_KEM_alg_sidh_p610,
    ("sidh") SidhP751: OQS_KEM_alg_sidh_p751,
    ("sidh") SidhP434Compressed: OQS_KEM_alg_sidh_p434_compressed,
    ("sidh") SidhP503Compressed: OQS_KEM_alg_sidh_p503_compressed,
    ("sidh") SidhP610Compressed: OQS_KEM_alg_sidh_p610_compressed,
    ("sidh") SidhP751Compressed: OQS_KEM_alg_sidh_p751_compressed,
    ("sike") SikeP434: OQS_KEM_alg_sike_p434,
    ("sike") SikeP503: OQS_KEM_alg_sike_p503,
    ("sike") SikeP610: OQS_KEM_alg_sike_p610,
    ("sike") SikeP751: OQS_KEM_alg_sike_p751,
    ("sike") SikeP434Compressed: OQS_KEM_alg_sike_p434_compressed,
    ("sike") SikeP503Compressed: OQS_KEM_alg_sike_p503_compressed,
    ("sike") SikeP610Compressed: OQS_KEM_alg_sike_p610_compressed,
    ("sike") SikeP751Compressed: OQS_KEM_alg_sike_p751_compressed,
    ("sike") SikeP434Compressed1CCA: OQS_KEM_alg_sike_p434_1cca_compressed,
    ("sike") SikeP503Compressed1CCA: OQS_KEM_alg_sike_p503_1cca_compressed,
    ("sike") SikeP610Compressed1CCA: OQS_KEM_alg_sike_p610_1cca_compressed,
    ("sike") SikeP751Compressed1CCA: OQS_KEM_alg_sike_p751_1cca_compressed,
    ("csidh") CsidhP512: OQS_KEM_alg_csidh_p512,

}

impl Algorithm {
    /// Returns true if this algorithm is enabled in the linked version
    /// of liboqs
    pub fn is_enabled(self) -> bool {
        unsafe { ffi::OQS_KEM_alg_is_enabled(algorithm_to_id(self)) == 1 }
    }

    /// Provides a pointer to the id of the algorithm
    ///
    /// For use with the FFI api methods
    pub fn to_id(self) -> *const libc::c_char {
        algorithm_to_id(self)
    }

    /// Returns the algorithm's name as a static Rust string.
    ///
    /// This is the same as the `to_id`, but as a safe Rust string.
    pub fn name(&self) -> &'static str {
        // SAFETY: The id from ffi must be a proper null terminated C string
        let id = unsafe { CStr::from_ptr(self.to_id()) };
        id.to_str().expect("OQS algorithm names must be UTF-8")
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name().fmt(f)
    }
}

/// KEM algorithm
///
/// # Example
/// ```rust
/// # if !cfg!(feature = "kyber") { return; }
/// use oqs;
/// oqs::init();
/// let kem = oqs::kem::Kem::new(oqs::kem::Algorithm::Kyber512).unwrap();
/// let (pk, sk) = kem.keypair().unwrap();
/// let (ct, ss) = kem.encapsulate(&pk).unwrap();
/// let ss2 = kem.decapsulate(&sk, &ct).unwrap();
/// assert_eq!(ss, ss2);
/// ```
pub struct Kem {
    algorithm: Algorithm,
    kem: NonNull<ffi::OQS_KEM>,
}

unsafe impl Sync for Kem {}
unsafe impl Send for Kem {}

impl Drop for Kem {
    fn drop(&mut self) {
        unsafe { ffi::OQS_KEM_free(self.kem.as_ptr()) };
    }
}

impl core::convert::TryFrom<Algorithm> for Kem {
    type Error = crate::Error;
    fn try_from(alg: Algorithm) -> Result<Kem> {
        Kem::new(alg)
    }
}

impl Kem {
    /// Construct a new algorithm
    pub fn new(algorithm: Algorithm) -> Result<Self> {
        let kem = unsafe { ffi::OQS_KEM_new(algorithm_to_id(algorithm)) };
        NonNull::new(kem).map_or_else(
            || Err(Error::AlgorithmDisabled),
            |kem| Ok(Self { algorithm, kem }),
        )
    }

    /// Get the algorithm used by this `Kem`
    pub fn algorithm(&self) -> Algorithm {
        self.algorithm
    }

    /// Get the version of the implementation
    pub fn version(&self) -> &'static str {
        let kem = unsafe { self.kem.as_ref() };
        // SAFETY: The alg_version from ffi must be a proper null terminated C string
        let cstr = unsafe { CStr::from_ptr(kem.alg_version) };
        cstr.to_str()
            .expect("Algorithm version strings must be UTF-8")
    }

    /// Get the claimed nist level
    pub fn claimed_nist_level(&self) -> u8 {
        let kem = unsafe { self.kem.as_ref() };
        kem.claimed_nist_level
    }

    /// Is the algorithm ind_cca secure
    pub fn is_ind_cca(&self) -> bool {
        let kem = unsafe { self.kem.as_ref() };
        kem.ind_cca
    }

    /// Get the length of the public key
    pub fn length_public_key(&self) -> usize {
        let kem = unsafe { self.kem.as_ref() };
        kem.length_public_key
    }

    /// Get the length of the secret key
    pub fn length_secret_key(&self) -> usize {
        let kem = unsafe { self.kem.as_ref() };
        kem.length_secret_key
    }

    /// Get the length of the ciphertext
    pub fn length_ciphertext(&self) -> usize {
        let kem = unsafe { self.kem.as_ref() };
        kem.length_ciphertext
    }

    /// Get the length of a shared secret
    pub fn length_shared_secret(&self) -> usize {
        let kem = unsafe { self.kem.as_ref() };
        kem.length_shared_secret
    }

    /// Get the length of a shared secret
    pub fn length_ephemeral_secret(&self) -> usize {
        let kem = unsafe { self.kem.as_ref() };
        kem.length_ephemeral_secret
    }

    /// Obtain a secret key objects from bytes
    ///
    /// Returns None if the secret key is not the correct length.
    pub fn secret_key_from_bytes<'a>(&self, buf: &'a [u8]) -> Option<SecretKeyRef<'a>> {
        if self.length_secret_key() != buf.len() {
            None
        } else {
            Some(SecretKeyRef::new(buf))
        }
    }

    /// Obtain a public key from bytes
    ///
    /// Returns None if the public key is not the correct length.
    pub fn public_key_from_bytes<'a>(&self, buf: &'a [u8]) -> Option<PublicKeyRef<'a>> {
        if self.length_public_key() != buf.len() {
            None
        } else {
            Some(PublicKeyRef::new(buf))
        }
    }

    /// Obtain a ciphertext from bytes
    ///
    /// Returns None if the ciphertext is not the correct length.
    pub fn ciphertext_from_bytes<'a>(&self, buf: &'a [u8]) -> Option<CiphertextRef<'a>> {
        if self.length_ciphertext() != buf.len() {
            None
        } else {
            Some(CiphertextRef::new(buf))
        }
    }

    /// Obtain a secret key from bytes
    ///
    /// Returns None if the shared secret is not the correct length.
    pub fn shared_secret_from_bytes<'a>(&self, buf: &'a [u8]) -> Option<SharedSecretRef<'a>> {
        if self.length_shared_secret() != buf.len() {
            None
        } else {
            Some(SharedSecretRef::new(buf))
        }
    }

    /// Obtain a secret key from bytes
    ///
    /// Returns None if the shared secret is not the correct length.
    pub fn ephemeral_secret_from_bytes<'a>(&self, buf: &'a [u8]) -> Option<EphemeralSecretRef<'a>> {
        if self.length_ephemeral_secret() != buf.len() {
            None
        } else {
            Some(EphemeralSecretRef::new(buf))
        }
    }

    /// Initialize the KEM
    pub fn init(&self) -> Result<()> {
        let kem = unsafe { self.kem.as_ref() };
        let func = kem.init.unwrap();
        let status = unsafe { func() };
        status_to_result(status)?;

        Ok(())
    }

    /// Uninitialize the KEM
    pub fn deinit(&self) -> Result<()> {
        let kem = unsafe { self.kem.as_ref() };
        let func = kem.deinit.unwrap();
        let status = unsafe { func() };
        status_to_result(status)?;

        Ok(())
    }

    /// Generate a new keypair
    pub fn keypair(&self) -> Result<(PublicKey, SecretKey)> {
        let kem = unsafe { self.kem.as_ref() };
        let func = kem.keypair.unwrap();
        let mut pk = PublicKey {
            bytes: Vec::with_capacity(kem.length_public_key),
        };
        let mut sk = SecretKey {
            bytes: Vec::with_capacity(kem.length_secret_key),
        };
        let status = unsafe { func(pk.bytes.as_mut_ptr(), sk.bytes.as_mut_ptr()) };
        status_to_result(status)?;
        // update the lengths of the vecs
        // this is safe to do, as we have initialised them now.
        unsafe {
            pk.bytes.set_len(kem.length_public_key);
            sk.bytes.set_len(kem.length_secret_key);
        }
        Ok((pk, sk))
    }

    /// Generate a new keypair
    pub fn keypair_async(&self) -> Result<(PublicKey, SecretKey)> {
        let kem = unsafe { self.kem.as_ref() };
        let func = kem.keypair_async.unwrap();
        let mut pk = PublicKey {
            bytes: Vec::with_capacity(kem.length_public_key),
        };
        let mut sk = SecretKey {
            bytes: Vec::with_capacity(kem.length_secret_key),
        };
        let status = unsafe { func(pk.bytes.as_mut_ptr(), sk.bytes.as_mut_ptr()) };
        status_to_result(status)?;
        // update the lengths of the vecs
        // this is safe to do, as we have initialised them now.
        unsafe {
            pk.bytes.set_len(kem.length_public_key);
            sk.bytes.set_len(kem.length_secret_key);
        }
        Ok((pk, sk))
    }

    /// Encapsulate to the provided public key
    pub fn encapsulate<'a, P: Into<PublicKeyRef<'a>>>(
        &self,
        pk: P,
    ) -> Result<(Ciphertext, SharedSecret)> {
        let pk = pk.into();
        if pk.bytes.len() != self.length_public_key() {
            return Err(Error::InvalidLength);
        }
        let kem = unsafe { self.kem.as_ref() };
        let func = kem.encaps.unwrap();
        let mut ct = Ciphertext {
            bytes: Vec::with_capacity(kem.length_ciphertext),
        };
        let mut ss = SharedSecret {
            bytes: Vec::with_capacity(kem.length_shared_secret),
        };
        // call encapsulate
        let status = unsafe {
            func(
                ct.bytes.as_mut_ptr(),
                ss.bytes.as_mut_ptr(),
                pk.bytes.as_ptr(),
            )
        };
        status_to_result(status)?;
        // update the lengths of the vecs
        // this is safe to do, as we have initialised them now.
        unsafe {
            ct.bytes.set_len(kem.length_ciphertext);
            ss.bytes.set_len(kem.length_shared_secret);
        }
        Ok((ct, ss))
    }

    /// Async encapsulate to the provided public key
    pub fn async_encapsulate<'a, P: Into<PublicKeyRef<'a>>>(
        &self,
        pk: P,
    ) -> Result<(Ciphertext, SharedSecret)> {
        let pk = pk.into();
        if pk.bytes.len() != self.length_public_key() {
            return Err(Error::InvalidLength);
        }
        let kem = unsafe { self.kem.as_ref() };
        let func = kem.async_encaps.unwrap();
        let mut ct = Ciphertext {
            bytes: Vec::with_capacity(kem.length_ciphertext),
        };
        let mut ss = SharedSecret {
            bytes: Vec::with_capacity(kem.length_shared_secret),
        };
        // call encapsulate
        let status = unsafe {
            func(
                ct.bytes.as_mut_ptr(),
                ss.bytes.as_mut_ptr(),
                pk.bytes.as_ptr(),
            )
        };
        status_to_result(status)?;
        // update the lengths of the vecs
        // this is safe to do, as we have initialised them now.
        unsafe {
            ct.bytes.set_len(kem.length_ciphertext);
            ss.bytes.set_len(kem.length_shared_secret);
        }
        Ok((ct, ss))
    }


    /// Encapsulate to the provided public key
    pub fn encapsulate_ciphertext<'a, P: Into<PublicKeyRef<'a>>>(
        &self,
        pk: P,
    ) -> Result<(Ciphertext, EphemeralSecret)> {
        let pk = pk.into();
        if pk.bytes.len() != self.length_public_key() {
            return Err(Error::InvalidLength);
        }
        let kem = unsafe { self.kem.as_ref() };
        let func = kem.encaps_ciphertext.unwrap();
        let mut ct = Ciphertext {
            bytes: Vec::with_capacity(kem.length_ciphertext),
        };
        let mut es = EphemeralSecret {
            bytes: Vec::with_capacity(kem.length_ephemeral_secret),
        };
        // call encapsulate
        let status = unsafe {
            func(
                ct.bytes.as_mut_ptr(),
                es.bytes.as_mut_ptr(),
                pk.bytes.as_ptr(),
            )
        };
        status_to_result(status)?;
        // update the lengths of the vecs
        // this is safe to do, as we have initialised them now.
        unsafe {
            ct.bytes.set_len(kem.length_ciphertext);
            es.bytes.set_len(kem.length_ephemeral_secret);
        }
        Ok((ct, es))
    }

    pub fn encapsulate_shared_secret<'a, 
        P: Into<PublicKeyRef<'a>>,
        C: Into<CiphertextRef<'a>>,
        E: Into<EphemeralSecretRef<'a>>,
    >(
        &self,
        ct: C,
        es: E,
        pk: P,
    ) -> Result<SharedSecret> {
        let pk = pk.into();
        if pk.bytes.len() != self.length_public_key() {
            return Err(Error::InvalidLength);
        }
        let kem = unsafe { self.kem.as_ref() };

        let ct = ct.into();
        if ct.bytes.len() != kem.length_ciphertext {
            return Err(Error::InvalidLength);
        }
        let es = es.into();
        if es.bytes.len() != kem.length_ephemeral_secret {
            return Err(Error::InvalidLength);
        }

        let func = kem.encaps_shared_secret.unwrap();
        let mut ss = SharedSecret {
            bytes: Vec::with_capacity(kem.length_shared_secret),
        };
        // call encapsulate
        let status = unsafe {
            func(
                ss.bytes.as_mut_ptr(),
                ct.bytes.as_ptr(),
                es.bytes.as_ptr(),
                pk.bytes.as_ptr(),
            )
        };
        status_to_result(status)?;
        // update the lengths of the vecs
        // this is safe to do, as we have initialised them now.
        unsafe {
            ss.bytes.set_len(kem.length_shared_secret);
        }
        Ok(ss)
    }

    /// Decapsulate the provided ciphertext
    pub fn decapsulate<'a, 'b, S: Into<SecretKeyRef<'a>>, C: Into<CiphertextRef<'b>>>(
        &self,
        sk: S,
        ct: C,
    ) -> Result<SharedSecret> {
        let kem = unsafe { self.kem.as_ref() };
        let sk = sk.into();
        let ct = ct.into();
        if sk.bytes.len() != self.length_secret_key() || ct.bytes.len() != self.length_ciphertext()
        {
            return Err(Error::InvalidLength);
        }
        let mut ss = SharedSecret {
            bytes: Vec::with_capacity(kem.length_shared_secret),
        };
        let func = kem.decaps.unwrap();
        // Call decapsulate
        let status = unsafe { func(ss.bytes.as_mut_ptr(), ct.bytes.as_ptr(), sk.bytes.as_ptr()) };
        status_to_result(status)?;
        // update the lengths of the vecs
        // this is safe to do, as we have initialised them now.
        unsafe { ss.bytes.set_len(kem.length_shared_secret) };
        Ok(ss)
    }
}
