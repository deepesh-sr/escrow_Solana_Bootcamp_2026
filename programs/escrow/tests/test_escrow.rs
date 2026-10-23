use {
    anchor_lang::{AccountDeserialize, AnchorDeserialize, InstructionData, ToAccountMetas, prelude::system_program, solana_program::{instruction::Instruction, pubkey::Pubkey}}, escrow::Escrow, litesvm::LiteSVM, solana_keypair::Keypair, solana_message::{Message, VersionedMessage}, solana_signer::Signer, solana_transaction::versioned::VersionedTransaction
};

#[test]
fn test_escrow() {
    let program_id = escrow::id();
    let maker = Keypair::new();
    let taker = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/escrow.so");

    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&maker.pubkey(), 1_000_000_000).unwrap();
    svm.airdrop(&taker.pubkey(), 1_000_000_000).unwrap();
    
    let seed : u32 = 1 ; 
    let (escrow_pda , _escrow_bump) = Pubkey::find_program_address(
        &[b"escrow" , maker.pubkey().as_ref(), &seed.to_le_bytes()], &program_id);

    let (vault_pda , _vault_bump) = Pubkey::find_program_address(
        &[b"vault", escrow_pda.as_ref()], &program_id);
    
    
    let instruction = Instruction::new_with_bytes(
        program_id,
        &escrow::instruction::Make {
            amount : 64000,
            seed : 1,
            taker : taker.pubkey()
        }.data(),
        escrow::accounts::Make {
            escrow_pda,
            maker : maker.pubkey(),
            vault : vault_pda,
            system_program : system_program::ID
        }.to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&maker.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[maker]).unwrap();

    let res = svm.send_transaction(tx);

    // vault account 
    let vault_account = svm.get_account(&vault_pda).expect("Vault should exist");
    assert_eq!(vault_account.lamports, 64000) ;

    // escrow account 
    let escrow_pda_account = svm.get_account(&escrow_pda).expect("Escrow Pda should exist"); 
    let mut data = escrow_pda_account.data.as_slice();
    let escrow_state = Escrow::try_deserialize(&mut data).expect("Deseriealize Escrow"); 

    assert_eq!(escrow_state.amount, 64000); 
    assert_eq!(escrow_state.seed, 1); 
    assert_eq!(escrow_state.taker, taker.pubkey()); 

    let taker_lamports_before = svm.get_balance(&taker.pubkey());

    let instruction = Instruction::new_with_bytes(
        program_id,
        &escrow::instruction::Take {}.data(),
        escrow::accounts::Take {
            taker : taker.pubkey(),
            maker : escrow_state.maker,
            escrow_pda,
            vault : vault_pda,
            system_program : system_program::ID
        }.to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&taker.pubkey()), &blockhash);
    let fees = 5_000 * msg.header.num_required_signatures as u64; 
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[taker]).unwrap();

    let res = svm.send_transaction(tx);


    // let taker_account = svm.get_account(&escrow_state.taker).expect("taker should exist");
    let taker_lamports_after = svm.get_balance(&escrow_state.taker);
    assert_eq!(taker_lamports_after.unwrap(), taker_lamports_before.unwrap() + 64000 - fees); 


    assert!(svm.get_account(&escrow_pda).is_none());
    assert!(svm.get_account(&vault_pda).is_none());


}
