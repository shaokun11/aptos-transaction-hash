use std::{env, fs};
use std::io::Write;
use aptos_crypto::ed25519::Ed25519PrivateKey;
use aptos_crypto::{PrivateKey, Uniform, ValidCryptoMaterialStringExt};
use aptos_types::transaction::SignedTransaction;
use rand::rngs::StdRng;
use rand_core::SeedableRng;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let k1 = vec![1u8; 32];
    // let key = Ed25519PrivateKey::try_from(k1.as_slice()).unwrap();
    // println!("--k1:{}", hex::encode(key.to_bytes()));
    // println!("--k1-pub:{}", hex::encode(key.public_key().to_bytes()));
    //
    //
    // let genesis_seed: [u8; 32] = [42; 32];
    // let mut rng = StdRng::from_seed(genesis_seed);
    // let private_key = Ed25519PrivateKey::generate(&mut rng);
    // println!("--k2:{}", private_key.to_encoded_string().unwrap());
    // println!("--k2-pub :{}", hex::encode(private_key.public_key().to_bytes()));
    //
    // let serialized_keys = bcs::to_bytes(&private_key).unwrap();
    // let aptos_root_key_path = "mint.key";
    // let mut key_file = fs::File::create(&aptos_root_key_path).unwrap();
    // key_file.write_all(&serialized_keys).unwrap();
    // let x = "b3e5e9d58797efbce688894c9aebf09afb074d9c03201b452bc81e8afcd4a75d9b020000000000000200000000000000000000000000000000000000000000000000000000000000010365766d076465706f73697400021514f99be3213562e5a381abb02818f1321304b44eaa07065af3107a4000400d03000000000064000000000000004877826500000000040020d91cd0f918bcf87fa5b1969dbe21af5973de6abbc1eced010f866e4a19dbeeca402031f26d165ef51ac75f8ebacf62d2c62ea454c3f0c67fa4c956a4e2d2e4a5fdb2fd0923287e9e2d1ebd4b314cdca237de3e925870ac52be0751ed837ceb300c";
    let x = args.get(1).unwrap().as_str();
    let s = bcs::from_bytes::<SignedTransaction>(&hex::decode(x).unwrap());
    println!("{}",hex::encode(s.unwrap().committed_hash().to_vec()))

}
