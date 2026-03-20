/*
    To run this benchmark:
    `cargo bench --bench bls_vote_sigverify`
*/

use {
    agave_votor_messages::{consensus_message::VoteMessage, vote::Vote},
    criterion::{BatchSize, Criterion, criterion_group, criterion_main},
    rayon::{ThreadPool, ThreadPoolBuilder, iter::IntoParallelIterator},
    solana_bls_signatures::{
        Keypair as BLSKeypair, PreparedHashedMessage, Pubkey as BLSPubkey, VerifiablePubkey,
        pubkey::PubkeyProjective, signature::SignatureProjective,
    },
    solana_core::bls_sigverify::{
        bls_vote_sigverify::{
            VotePayload, aggregate_pubkeys_by_payload, aggregate_signatures,
            verify_individual_votes, verify_votes_optimistic,
        },
        stats::SigVerifyVoteStats,
    },
    solana_hash::Hash,
    solana_keypair::Keypair,
    solana_signer::Signer,
    std::{collections::HashMap, hint::black_box, sync::Arc},
};

static MESSAGE_COUNTS: &[usize] = &[1, 2, 4, 8, 16];
static BATCH_SIZES: &[usize] = &[8, 16, 32, 64, 128];
static TOTAL_COUNTS: &[usize] = &[128, 256];

fn get_thread_pool() -> ThreadPool {
    let num_threads = 4;
    ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap()
}

fn get_matrix_params() -> impl Iterator<Item = (usize, usize)> {
    BATCH_SIZES.iter().flat_map(|&batch_size| {
        MESSAGE_COUNTS.iter().filter_map(move |&num_distinct| {
            if num_distinct > batch_size {
                None
            } else {
                Some((batch_size, num_distinct))
            }
        })
    })
}

fn get_cache_matrix_params() -> impl Iterator<Item = (usize, usize)> {
    TOTAL_COUNTS.iter().flat_map(|&n| {
        let ks = [2, n / 4, n / 2, n];
        ks.into_iter().map(move |k| (n, k))
    })
}

fn generate_test_data(num_distinct_messages: usize, batch_size: usize) -> Vec<VotePayload> {
    assert!(
        batch_size >= num_distinct_messages,
        "Batch size must be >= distinct messages"
    );

    // Pre-calculate the payloads to ensure exact distinctness
    let base_payloads: Vec<Arc<Vec<u8>>> = (0..num_distinct_messages)
        .map(|i| {
            let slot = (i as u64).saturating_add(100);
            let vote = Vote::new_notarization_vote(slot, Hash::new_unique());
            Arc::new(bincode::serialize(&vote).unwrap())
        })
        .collect();

    let mut votes_to_verify = Vec::with_capacity(batch_size);

    for i in 0..batch_size {
        let payload = &base_payloads[i.rem_euclid(num_distinct_messages)];

        let bls_keypair = BLSKeypair::new();
        let vote: Vote = bincode::deserialize(payload).unwrap();

        let signature = bls_keypair.sign(payload);

        let vote_message = VoteMessage {
            vote,
            signature: signature.into(),
            rank: 0,
        };

        votes_to_verify.push(VotePayload {
            vote_message,
            bls_pubkey: bls_keypair.public,
            pubkey: Keypair::new().pubkey(),
            remote_pubkey: Keypair::new().pubkey(),
        });
    }

    votes_to_verify
}

fn precompute_prepared_payloads(votes: &[VotePayload]) -> HashMap<Vote, PreparedHashedMessage> {
    votes
        .iter()
        .fold(HashMap::with_capacity(votes.len()), |mut acc, vote| {
            acc.entry(vote.vote_message.vote).or_insert_with(|| {
                let payload = wincode::serialize(&vote.vote_message.vote).unwrap();
                PreparedHashedMessage::new(&payload)
            });
            acc
        })
}

fn verify_votes_optimistic_with_prepared_payloads(
    votes: &[VotePayload],
    prepared_payloads: Option<&HashMap<Vote, PreparedHashedMessage>>,
    thread_pool: &ThreadPool,
) -> bool {
    let use_cached_payloads = prepared_payloads.is_some();
    let (signature_result, (distinct_payloads, aggregate_pubkeys)) = thread_pool.join(
        || aggregate_signatures(votes),
        || {
            let mut grouped_votes: HashMap<&Vote, Vec<_>> = HashMap::new();
            let mut inline_prepared_payloads: HashMap<&Vote, PreparedHashedMessage> =
                HashMap::new();
            for vote in votes {
                grouped_votes
                    .entry(&vote.vote_message.vote)
                    .or_default()
                    .push(&vote.bls_pubkey);

                if !use_cached_payloads {
                    let payload = wincode::serialize(&vote.vote_message.vote).unwrap();
                    let prepared_payload = PreparedHashedMessage::new(&payload);
                    inline_prepared_payloads
                        .entry(&vote.vote_message.vote)
                        .or_insert(prepared_payload);
                }
            }

            let mut grouped_payloads = Vec::with_capacity(grouped_votes.len());
            let mut grouped_pubkeys = Vec::with_capacity(grouped_votes.len());
            for (vote, pubkeys) in grouped_votes {
                let prepared_payload = match prepared_payloads {
                    Some(payloads) => payloads
                        .get(vote)
                        .expect("precomputed payload should exist for all votes")
                        .clone(),
                    None => inline_prepared_payloads
                        .remove(vote)
                        .expect("prepared payload should be computed inline"),
                };
                grouped_payloads.push(prepared_payload);
                grouped_pubkeys.push(
                    PubkeyProjective::par_aggregate(pubkeys.into_par_iter())
                        .expect("pubkey aggregation should succeed"),
                );
            }
            (grouped_payloads, grouped_pubkeys)
        },
    );

    let Ok(aggregate_signature) = signature_result else {
        return false;
    };

    if distinct_payloads.len() != aggregate_pubkeys.len() {
        return false;
    }

    if aggregate_pubkeys.len() == 1 {
        aggregate_pubkeys[0]
            .verify_signature_prepared(&aggregate_signature, &distinct_payloads[0])
            .is_ok()
    } else {
        SignatureProjective::par_verify_distinct_aggregated_prepared(
            &aggregate_pubkeys,
            &aggregate_signature,
            &distinct_payloads,
        )
        .is_ok()
    }
}

// Single Signature Verification
// This is just for reference
fn bench_verify_single_signature(c: &mut Criterion) {
    let mut group = c.benchmark_group("verify_single_signature");

    let keypair = BLSKeypair::new();
    let msg = b"benchmark_message_payload";
    let sig = keypair.sign(msg);
    let pubkey: BLSPubkey = keypair.public.into();
    let prepared_msg = PreparedHashedMessage::new(msg);

    group.bench_function("1_item", |b| {
        b.iter(|| {
            let res = pubkey.verify_signature_prepared(black_box(&sig), black_box(&prepared_msg));
            black_box(res).unwrap();
        })
    });
    group.finish();
}

// Optimistic Verification - aggregates the public keys and signatures first before verifying.
// Depends on both batch size and message distinctness due to pairing checks.
fn bench_verify_votes_optimistic(c: &mut Criterion) {
    let mut group = c.benchmark_group("verify_votes_optimistic");
    let mut stats = SigVerifyVoteStats::default();
    let thread_pool = get_thread_pool();

    for (batch_size, num_distinct) in get_matrix_params() {
        let votes = generate_test_data(num_distinct, batch_size);
        let label = format!("msgs_{num_distinct}/batch_{batch_size}");

        group.bench_function(&label, |b| {
            b.iter(|| {
                let res = verify_votes_optimistic(black_box(&votes), &mut stats, &thread_pool);
                black_box(res);
            })
        });
    }
    group.finish();
    black_box(stats);
}

// Public Key Aggregation
// Depends on message distinctness because keys are grouped by messages.
fn bench_aggregate_pubkeys(c: &mut Criterion) {
    let mut group = c.benchmark_group("aggregate_pubkeys");
    let mut stats = SigVerifyVoteStats::default();

    for (batch_size, num_distinct) in get_matrix_params() {
        let votes = generate_test_data(num_distinct, batch_size);
        let label = format!("msgs_{num_distinct}/batch_{batch_size}");

        group.bench_function(&label, |b| {
            b.iter(|| {
                let res = aggregate_pubkeys_by_payload(black_box(&votes), &mut stats);
                black_box(res).1.unwrap();
            })
        });
    }
    group.finish();
    black_box(stats);
}

// Signature Aggregation
// Pure G1 addition - message distinctness is irrelevant.
fn bench_aggregate_signatures(c: &mut Criterion) {
    let mut group = c.benchmark_group("aggregate_signatures");

    for &batch_size in BATCH_SIZES {
        // Use 1 distinct message just to generate valid data cheaply.
        // It doesn't affect signature aggregation performance.
        let votes = generate_test_data(1, batch_size);
        let label = format!("batch_{batch_size}");

        group.bench_function(&label, |b| {
            b.iter(|| {
                let res = aggregate_signatures(black_box(&votes));
                black_box(res).unwrap();
            })
        });
    }
    group.finish();
}

// Optimistic verification with/without precomputed prepared messages.
fn bench_verify_votes_optimistic_with_cache(c: &mut Criterion) {
    let mut group = c.benchmark_group("verify_votes_optimistic_cache");
    let thread_pool = get_thread_pool();

    // n: total number of (pk, sig, h(msg))
    // k: number of unique h(msg)
    for (n, k) in get_cache_matrix_params() {
        let votes = generate_test_data(k, n);
        let base_label = format!("n={n}/k={k}");

        let no_cache_label = format!("{base_label}/nocache_e2e");
        group.bench_function(&no_cache_label, |b| {
            b.iter(|| {
                let res = verify_votes_optimistic_with_prepared_payloads(
                    black_box(&votes),
                    None,
                    &thread_pool,
                );
                black_box(res);
            })
        });

        // E2E cached path: includes preprocessing + grouping + multi-distinct verify.
        let cache_label = format!("{base_label}/cached_e2e");
        group.bench_function(&cache_label, |b| {
            b.iter(|| {
                let prepared_payloads = precompute_prepared_payloads(&votes);
                let res = verify_votes_optimistic_with_prepared_payloads(
                    black_box(&votes),
                    Some(black_box(&prepared_payloads)),
                    &thread_pool,
                );
                black_box(res);
            })
        });
    }
    group.finish();
}

// Individual Verification - verifies each signatures in parallel threads
// Message distinctness is irrelevant.
fn bench_verify_individual_votes(c: &mut Criterion) {
    let mut group = c.benchmark_group("verify_votes_fallback");
    let thread_pool = get_thread_pool();

    for &batch_size in BATCH_SIZES {
        // Distinctness doesn't affect the cost of N individual verifications.
        let votes = generate_test_data(1, batch_size);
        let label = format!("batch_{batch_size}");

        group.bench_function(&label, |b| {
            b.iter_batched(
                || votes.clone(),
                |votes| {
                    let res = verify_individual_votes(black_box(votes), &thread_pool);
                    black_box(res);
                },
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_verify_single_signature,
    bench_verify_votes_optimistic,
    bench_verify_votes_optimistic_with_cache,
    bench_aggregate_pubkeys,
    bench_aggregate_signatures,
    bench_verify_individual_votes
);
criterion_main!(benches);
