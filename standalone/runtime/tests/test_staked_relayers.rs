mod mock;

use crate::redeem_testing_utils::{setup_redeem, USER};
use currency::Amount;
use mock::{assert_eq, *};
use primitives::VaultCurrencyPair;
use sp_core::H256;

pub const RELAYER: [u8; 32] = ALICE;
pub const VAULT: [u8; 32] = BOB;

fn test_with<R>(execute: impl Fn(CurrencyId) -> R) {
    let test_with = |currency_id| {
        ExtBuilder::build().execute_with(|| {
            assert_ok!(OraclePallet::_set_exchange_rate(currency_id, FixedU128::one()));
            UserData::force_to(USER, default_user_state());
            execute(currency_id)
        })
    };
    test_with(CurrencyId::DOT);
    test_with(CurrencyId::KSM);
}

#[test]
fn integration_test_report_vault_theft() {
    test_with(|currency_id| {
        let user = ALICE;
        let vault = BOB;
        let theft_amount = wrapped(100);
        let collateral_vault = 1000000;
        let issued_tokens = wrapped(100);
        let vault_id = vault_id_of(vault, currency_id);

        let vault_btc_address = BtcAddress::P2SH(H160([
            215, 255, 109, 96, 235, 244, 10, 155, 24, 134, 172, 206, 6, 101, 59, 162, 34, 77, 143, 234,
        ]));
        let other_btc_address = BtcAddress::P2SH(H160([1; 20]));

        SecurityPallet::set_active_block_number(1);

        assert_ok!(Call::VaultRegistry(VaultRegistryCall::register_vault(
            VaultCurrencyPair {
                collateral: currency_id,
                wrapped: DEFAULT_WRAPPED_CURRENCY,
            },
            collateral_vault,
            dummy_public_key(),
        ))
        .dispatch(origin_of(account_of(vault))));
        assert_ok!(VaultRegistryPallet::insert_vault_deposit_address(
            vault_id.clone(),
            vault_btc_address
        ));

        assert_ok!(VaultRegistryPallet::try_increase_to_be_issued_tokens(
            &vault_id,
            &issued_tokens,
        ));
        assert_ok!(VaultRegistryPallet::issue_tokens(&vault_id, &issued_tokens));

        let (_tx_id, _height, proof, raw_tx, _) = TransactionGenerator::new()
            .with_address(other_btc_address)
            .with_amount(theft_amount)
            .with_confirmations(7)
            .with_relayer(Some(ALICE))
            .mine();

        SecurityPallet::set_active_block_number(1000);

        let pre_liquidation_state = ParachainState::get(currency_id);
        let theft_fee = FeePallet::get_theft_fee(&Amount::new(collateral_vault, currency_id)).unwrap();

        assert_ok!(
            Call::Relay(RelayCall::report_vault_theft(vault_id, proof, raw_tx)).dispatch(origin_of(account_of(user)))
        );

        let confiscated_collateral = Amount::new(150, currency_id);
        assert_eq!(
            ParachainState::get(currency_id),
            pre_liquidation_state.with_changes(|user, vault, liquidation_vault, _fee_pool| {
                let liquidation_vault = liquidation_vault.with_currency(&currency_id);

                (*user.balances.get_mut(&currency_id).unwrap()).free += theft_fee;

                vault.issued -= issued_tokens;
                vault.backing_collateral -= confiscated_collateral;
                vault.backing_collateral -= theft_fee;

                liquidation_vault.issued += issued_tokens;
                liquidation_vault.collateral += confiscated_collateral;
            })
        );
    });
}

#[test]
fn integration_test_double_spend_op_return() {
    test_with(|_currency_id| {
        let issued_tokens = wrapped(10_000);
        // Register vault with hardcoded public key so it counts as theft
        let stealing_vault = DAVE;

        let vault_public_key_one = BtcPublicKey([
            2, 168, 49, 109, 0, 14, 227, 106, 112, 84, 59, 37, 153, 238, 121, 44, 66, 8, 181, 64, 248, 19, 137, 27, 47,
            222, 50, 95, 187, 221, 152, 165, 69,
        ]);

        let vault_public_key_two = BtcPublicKey([
            2, 139, 220, 235, 13, 249, 164, 152, 179, 4, 175, 217, 170, 84, 218, 179, 182, 247, 109, 48, 57, 152, 241,
            165, 225, 26, 242, 187, 160, 225, 248, 195, 250,
        ]);

        assert_ok!(Call::VaultRegistry(VaultRegistryCall::register_vault(
            VaultCurrencyPair {
                collateral: DEFAULT_TESTING_CURRENCY,
                wrapped: DEFAULT_WRAPPED_CURRENCY,
            },
            INITIAL_BALANCE,
            vault_public_key_one.clone(),
        ))
        .dispatch(origin_of(account_of(stealing_vault))));

        assert_ok!(VaultRegistryPallet::try_increase_to_be_issued_tokens(
            &default_vault_id_of(stealing_vault),
            &issued_tokens,
        ));
        assert_ok!(VaultRegistryPallet::issue_tokens(
            &default_vault_id_of(stealing_vault),
            &issued_tokens
        ));

        let redeem_id = setup_redeem(issued_tokens, USER, default_vault_id_of(stealing_vault));
        let redeem = RedeemPallet::get_open_redeem_request_from_id(&redeem_id).unwrap();
        let user_btc_address = BtcAddress::P2PKH(H160([2; 20]));
        let current_block_number = 1;

        // Send the honest redeem transaction
        let (_tx_id, _tx_block_height, merkle_proof, raw_tx) = {
            register_address_and_mine_transaction(
                default_vault_id_of(stealing_vault),
                vault_public_key_one,
                user_btc_address,
                redeem.amount_btc(),
                Some(redeem_id),
            )
        };

        // Double-spend the redeem, so the redeemer gets twice the BTC
        let (_theft_tx_id, _theft_tx_block_height, theft_merkle_proof, theft_raw_tx) =
            register_address_and_mine_transaction(
                default_vault_id_of(stealing_vault),
                vault_public_key_two,
                user_btc_address,
                redeem.amount_btc(),
                Some(redeem_id),
            );
        SecurityPallet::set_active_block_number(current_block_number + 1 + CONFIRMATIONS);

        assert_ok!(Call::Redeem(RedeemCall::execute_redeem(
            redeem_id,
            merkle_proof.clone(),
            raw_tx.clone()
        ))
        .dispatch(origin_of(account_of(stealing_vault))));

        // Executing the theft tx should fail
        assert_err!(
            Call::Redeem(RedeemCall::execute_redeem(
                redeem_id,
                theft_merkle_proof.clone(),
                theft_raw_tx.clone()
            ))
            .dispatch(origin_of(account_of(stealing_vault))),
            RedeemError::RedeemCompleted
        );

        // Reporting the double-spend transaction as theft should work
        assert_ok!(Call::Relay(RelayCall::report_vault_double_payment(
            default_vault_id_of(stealing_vault),
            (merkle_proof, theft_merkle_proof),
            (raw_tx, theft_raw_tx),
        ))
        .dispatch(origin_of(account_of(USER))));
    });
}

#[test]
fn integration_test_relay_parachain_status_check_fails() {
    ExtBuilder::build().execute_with(|| {
        SecurityPallet::set_status(StatusCode::Shutdown);

        assert_noop!(
            Call::Relay(RelayCall::initialize(Default::default(), 0)).dispatch(origin_of(account_of(ALICE))),
            SecurityError::ParachainShutdown
        );
        assert_noop!(
            Call::Relay(RelayCall::store_block_header(Default::default())).dispatch(origin_of(account_of(ALICE))),
            SecurityError::ParachainShutdown
        );
        assert_noop!(
            Call::Relay(RelayCall::report_vault_theft(
                vault_id_of(ALICE, DOT),
                Default::default(),
                Default::default()
            ))
            .dispatch(origin_of(account_of(ALICE))),
            SecurityError::ParachainShutdown
        );
    })
}
