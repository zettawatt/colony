use borsh::{BorshDeserialize, BorshSerialize};
use bip39::{Mnemonic, Language};
use autonomi::client::key_derivation::{DerivationIndex, MainSecretKey};
use autonomi::{SecretKey, PublicKey};
use cocoon::Cocoon;
use std::collections::HashMap;
use std::io::Error;

#[derive(BorshDeserialize, BorshSerialize)] // Ensure BorshSerialize is derived
struct SerializedSecretData {
    mnemonic: String,
    main_sk: Vec<u8>,
    pods: HashMap<Vec<u8>, Vec<u8>>,
}

impl From<SerializedSecretData> for SecretData {
    fn from(serialized: SerializedSecretData) -> Self {
        let mnemonic = Mnemonic::parse_in_normalized(Language::English, &serialized.mnemonic.as_str()).unwrap();
        let secret_key = SecretKey::from_bytes(serialized.main_sk.try_into().unwrap()).unwrap();
        let main_sk = MainSecretKey::new(secret_key);
        let pods = serialized
            .pods
            .into_iter()
            .map(|(k, v)| {
                let pub_key = PublicKey::from_bytes(k.try_into().unwrap()).unwrap();
                let sec_key = SecretKey::from_bytes(v.try_into().unwrap()).unwrap();
                (pub_key, sec_key)
            })
            .collect();

        SecretData {
            mnemonic,
            main_sk,
            pods,
        }
    }
}

impl From<SecretData> for SerializedSecretData {
    fn from(secret_data: SecretData) -> Self {
        let mnemonic = secret_data.mnemonic.to_string();
        let main_sk = secret_data.main_sk.to_bytes().to_vec();
        let pods = secret_data
            .pods
            .iter()
            .map(|(k, v)| (k.to_bytes().to_vec(), v.to_bytes().to_vec()))
            .collect();

        SerializedSecretData {
            mnemonic,
            main_sk,
            pods,
        }
    }
}

#[derive(Clone)]
pub struct SecretData {
    mnemonic: Mnemonic,
    main_sk: MainSecretKey,
    pods: HashMap<PublicKey, SecretKey>,
}

impl SecretData {
    pub fn from_file<R: std::io::Read>(file: &mut R, password: &str) -> Result<Self, cocoon::Error> {
        let cocoon = Cocoon::new(&password.as_bytes());
        let encoded = cocoon.parse(file)?;
        let deserialized = SerializedSecretData::try_from_slice(&encoded).unwrap();
        let secret_data: SecretData = deserialized.into();
        Ok(secret_data)
    }
    pub fn to_file<W: std::io::Write>(&self, file: &mut W, password: &str) -> Result<(), cocoon::Error> {
        let mut cocoon = Cocoon::new(&password.as_bytes());
        let serialized: SerializedSecretData = SerializedSecretData::from((*self).clone());
        let encoded = borsh::to_vec(&serialized).unwrap();
        cocoon.dump(encoded, file)?;
        Ok(())
    }
    pub fn from_mnemonic(mnemonic: String) -> Result<Self, Error> {

        // Generate a new mnemonic from the given phrase
        let mnemonic = Mnemonic::parse_in_normalized(Language::English, mnemonic.as_str()).unwrap();

        // Convert the mnemonic to a seed
        let seed = mnemonic.to_seed_normalized("");

        // Convert the seed to a BLS secret key
        let secret_key = SecretKey::from_bytes(seed[..32].try_into().unwrap_or_else(
            |error | {
                panic!("Problem grabbing the first 32 bytes of the seed: {:?}", error);
            }
        )).unwrap_or_else(|error| {
                panic!("Problem creating the secret key. Try running initialize again: {:?}", error);
            }
        );

        // Generate a new main keys from the mnemonic
        let main_sk: MainSecretKey = MainSecretKey::new(secret_key);
        //let main_pk: MainPubkey = main_sk.public_key();

        // Create a new pods hashmap and add the first pod
        let mut pods: HashMap<PublicKey, SecretKey> = HashMap::new();
        let pod_key: SecretKey = main_sk.derive_key(&index(0)).into();
        let pod_pubkey: PublicKey = pod_key.public_key();
        pods.insert(pod_pubkey, pod_key);

        Ok(SecretData {
            mnemonic: mnemonic,
            main_sk: main_sk,
            pods: pods,
        })
    }

    pub fn add_pod(&mut self, address: PublicKey, key: SecretKey) {
        self.pods.insert(address, key);
    }

    pub fn get_seed_phrase(&self) -> String {
        self.mnemonic.to_string()
    }

}

fn index(i: u64) -> DerivationIndex {
    let mut bytes = [0u8; 32];
    bytes[..8].copy_from_slice(&i.to_ne_bytes());
    DerivationIndex::from_bytes(bytes)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_data_from_mnemonic() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_string();
        let secret_data = SecretData::from_mnemonic(mnemonic.clone()).unwrap();

        assert_eq!(secret_data.get_seed_phrase(), mnemonic);
        assert!(secret_data.pods.len() > 0);
    }

    #[test]
    fn test_secret_data_to_and_from_file() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_string();
        let secret_data = SecretData::from_mnemonic(mnemonic).unwrap();

        let password = "test_password";
        let mut file = std::io::Cursor::new(Vec::new());

        secret_data.to_file(&mut file, password).unwrap();
        file.set_position(0);

        let loaded_secret_data = SecretData::from_file(&mut file, password).unwrap();

        assert_eq!(secret_data.get_seed_phrase(), loaded_secret_data.get_seed_phrase());
        assert_eq!(secret_data.pods.len(), loaded_secret_data.pods.len());
    }

    #[test]
    fn test_add_pod() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_string();
        let mut secret_data = SecretData::from_mnemonic(mnemonic).unwrap();

        let new_pod_key: SecretKey = secret_data.main_sk.derive_key(&index(1)).into();
        let new_pod_pubkey: PublicKey = new_pod_key.public_key();

        secret_data.add_pod(new_pod_pubkey, new_pod_key);

        assert!(secret_data.pods.contains_key(&new_pod_pubkey));
    }

    #[test]
    fn test_serialized_secret_data_conversion() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_string();
        let secret_data = SecretData::from_mnemonic(mnemonic).unwrap();

        let serialized: SerializedSecretData = secret_data.clone().into();
        let deserialized: SecretData = serialized.into();

        assert_eq!(secret_data.get_seed_phrase(), deserialized.get_seed_phrase());
        assert_eq!(secret_data.pods.len(), deserialized.pods.len());
    }
}
