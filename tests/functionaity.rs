use super::*;
use solana_program::clock::Epoch;
use std::mem;      
use {
    solana_program::{
        instruction::{AccountMeta, Instruction},
    },
    solana_program_test::*,
    solana_sdk::{account::Account, signature::Signer, transaction::Transaction},
    std::str::FromStr,
};

#[tokio::test]
async fn test_sanity() {
    let program_id = Pubkey::default();
    let from_key = Pubkey::new_unique();
    let to_key = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "Transer lamports test program", 
        program_id,
        processor!(process_instruction),
    );
    program_test.add_account(
        from_key,
        Account {
            lamports: 50,
            owner: program_id,
            ..Account::default()
        },
    );
    program_test.add_account(
        to_key, 
        Account {
            lamports: 50,
            ..Account::default()
        },
    );
    
    let(mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &(),
            vec![
                AccountMeta::new(from_key, false),
                AccountMeta::new(to_key, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}