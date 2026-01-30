#![feature(test)]

extern crate test;
use {
    agave_feature_set::FeatureSet,
    agave_precompiles::falcon::verify,
    rand0_7::{thread_rng, Rng},
    solana_falcon_signature::{
        new_falcon512_instruction_with_signature, SecretKey, DATA_START, MAX_SIGNATURE_SIZE,
        PUBKEY_SIZE,
    },
    solana_instruction::Instruction,
    test::Bencher,
};

// 5K instructions should be enough for benching loop
const IX_COUNT: u16 = 5120;

// prepare a bunch of unique txs
fn create_test_instructions(message_length: u16) -> Vec<Instruction> {
    (0..IX_COUNT)
        .map(|_| {
            let mut rng = thread_rng();
            let secret = SecretKey::generate().expect("key generation failed");
            let message: Vec<u8> = (0..message_length).map(|_| rng.gen_range(0, 255)).collect();
            let signature = secret.sign(&message).expect("signing failed");
            let pubkey = secret.public_key();
            new_falcon512_instruction_with_signature(&message, &signature, pubkey)
        })
        .collect()
}

#[bench]
fn bench_falcon_len_032(b: &mut Bencher) {
    let feature_set = FeatureSet::all_enabled();
    let ixs = create_test_instructions(32);
    let mut ix_iter = ixs.iter().cycle();
    b.iter(|| {
        let instruction = ix_iter.next().unwrap();
        verify(&instruction.data, &[&instruction.data], &feature_set).unwrap();
    });
}

#[bench]
fn bench_falcon_len_128(b: &mut Bencher) {
    let feature_set = FeatureSet::all_enabled();
    let ixs = create_test_instructions(128);
    let mut ix_iter = ixs.iter().cycle();
    b.iter(|| {
        let instruction = ix_iter.next().unwrap();
        verify(&instruction.data, &[&instruction.data], &feature_set).unwrap();
    });
}

#[bench]
fn bench_falcon_len_1k(b: &mut Bencher) {
    let feature_set = FeatureSet::all_enabled();
    let ixs = create_test_instructions(1024);
    let mut ix_iter = ixs.iter().cycle();
    b.iter(|| {
        let instruction = ix_iter.next().unwrap();
        verify(&instruction.data, &[&instruction.data], &feature_set).unwrap();
    });
}

#[bench]
fn bench_falcon_len_max(b: &mut Bencher) {
    let required_extra_space =
        DATA_START.saturating_add(PUBKEY_SIZE).saturating_add(MAX_SIGNATURE_SIZE);
    let message_len = (u16::MAX as usize).saturating_sub(required_extra_space) as u16;
    let feature_set = FeatureSet::all_enabled();
    let ixs = create_test_instructions(message_len);
    let mut ix_iter = ixs.iter().cycle();
    b.iter(|| {
        let instruction = ix_iter.next().unwrap();
        verify(&instruction.data, &[&instruction.data], &feature_set).unwrap();
    });
}
