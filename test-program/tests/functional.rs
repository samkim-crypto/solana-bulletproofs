#![cfg(feature = "test-sbf")]

use {
    boomerang_test_program::*,
    solana_program_test::*,
    solana_ristretto::{ristretto::RistrettoPoint, scalar::Scalar},
    solana_sdk::{signature::Signer, transaction::Transaction},
};

fn program_test() -> ProgramTest {
    ProgramTest::new(
        "boomerang_test_program",
        id(),
        processor!(process_instruction),
    )
}

#[tokio::test]
async fn test_transcript() {
    let mut context = program_test().start_with_context().await;

    let test_message = b"test_message".to_vec();
    let test_u64 = 77_u64;

    let transaction = Transaction::new_signed_with_payer(
        &[transcript(test_message, test_u64)],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_add_ristretto() {
    let mut context = program_test().start_with_context().await;

    let left_point = RistrettoPoint::from_bytes(&[
        208, 165, 125, 204, 2, 100, 218, 17, 170, 194, 23, 9, 102, 156, 134, 136, 217, 190, 98, 34,
        183, 194, 228, 153, 92, 11, 108, 103, 28, 57, 88, 15,
    ])
    .unwrap();
    let right_point = RistrettoPoint::from_bytes(&[
        208, 241, 72, 163, 73, 53, 32, 174, 54, 194, 71, 8, 70, 181, 244, 199, 93, 147, 99, 231,
        162, 127, 25, 40, 39, 19, 140, 132, 112, 212, 145, 108,
    ])
    .unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[add_ristretto(&left_point, &right_point)],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_subtract_ristretto() {
    let mut context = program_test().start_with_context().await;

    let left_point = RistrettoPoint::from_bytes(&[
        208, 165, 125, 204, 2, 100, 218, 17, 170, 194, 23, 9, 102, 156, 134, 136, 217, 190, 98, 34,
        183, 194, 228, 153, 92, 11, 108, 103, 28, 57, 88, 15,
    ])
    .unwrap();
    let right_point = RistrettoPoint::from_bytes(&[
        208, 241, 72, 163, 73, 53, 32, 174, 54, 194, 71, 8, 70, 181, 244, 199, 93, 147, 99, 231,
        162, 127, 25, 40, 39, 19, 140, 132, 112, 212, 145, 108,
    ])
    .unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[subtract_ristretto(&left_point, &right_point)],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_multiply_ristretto() {
    let mut context = program_test().start_with_context().await;

    let point = RistrettoPoint::from_bytes(&[
        208, 165, 125, 204, 2, 100, 218, 17, 170, 194, 23, 9, 102, 156, 134, 136, 217, 190, 98, 34,
        183, 194, 228, 153, 92, 11, 108, 103, 28, 57, 88, 15,
    ])
    .unwrap();
    let scalar = Scalar::from_bytes(&[
        8, 161, 219, 155, 192, 137, 153, 26, 27, 40, 30, 17, 124, 194, 26, 41, 32, 7, 161, 45, 212,
        198, 212, 81, 133, 185, 164, 85, 95, 232, 106, 10,
    ])
    .unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[multiply_ristretto(&point, &scalar)],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_multiscalar_multiply_ristretto() {
    let mut context = program_test().start_with_context().await;

    let scalar_a = Scalar::from_bytes(&[
        8, 161, 219, 155, 192, 137, 153, 26, 27, 40, 30, 17, 124, 194, 26, 41, 32, 7, 161, 45, 212,
        198, 212, 81, 133, 185, 164, 85, 95, 232, 106, 10,
    ])
    .unwrap();
    let scalar_b = Scalar::from_bytes(&[
        135, 207, 106, 208, 107, 127, 46, 82, 66, 22, 136, 125, 105, 62, 69, 34, 213, 210, 17, 196,
        120, 114, 238, 237, 149, 170, 5, 243, 54, 77, 172, 12,
    ])
    .unwrap();
    let point_x = RistrettoPoint::from_bytes(&[
        130, 35, 97, 25, 18, 199, 33, 239, 85, 143, 119, 111, 49, 51, 224, 40, 167, 185, 240, 179,
        25, 194, 213, 41, 14, 155, 104, 18, 181, 197, 15, 112,
    ])
    .unwrap();
    let point_y = RistrettoPoint::from_bytes(&[
        152, 156, 155, 197, 152, 232, 92, 206, 219, 159, 193, 134, 121, 128, 139, 36, 56, 191, 51,
        143, 72, 204, 87, 76, 110, 124, 101, 96, 238, 158, 42, 108,
    ])
    .unwrap();

    let scalars = vec![scalar_a, scalar_b];
    let points = vec![point_x, point_y];

    let transaction = Transaction::new_signed_with_payer(
        &[multiscalar_multiply_ristretto(scalars, points)],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();
}
