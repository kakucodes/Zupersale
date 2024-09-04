use cosmwasm_std::{coin, Decimal};
use cw_utils::NativeBalance;

use crate::{
    msg::{IncentiveCriteria, Reward, TokenAirdropDistributionType},
    utils::{funds_match_balance, required_incentives_tokens},
};

#[test]
fn funds_match_balance_test() {
    let required_tokens =
        NativeBalance::default() + coin(100, "ubtc") + coin(1_000_000_000, "uusdc");

    assert!(
        funds_match_balance(
            &required_tokens,
            &vec![coin(1_100, "ubtc"), coin(1_000_000_000, "uusdc")],
        )
        .is_err(),
        "the passed in funds are too large to match the required funds"
    );

    assert!(
        funds_match_balance(
            &required_tokens,
            &vec![coin(10, "ubtc"), coin(1_000_000_000, "uusdc")],
        )
        .is_err(),
        "the passed in funds are too small to match the required funds"
    );

    assert!(
        funds_match_balance(&required_tokens, &vec![coin(10, "ubtc")],).is_err(),
        "the passed in funds are missing a denom so they dont't match the required funds"
    );

    assert!(
        funds_match_balance(&required_tokens, &vec![],).is_err(),
        "the passed in funds are missing a denom so they dont't match the required funds"
    );

    assert!(
        funds_match_balance(
            &required_tokens,
            &vec![coin(10, "ubtc"), coin(10, "ueth"), coin(1_000_000_000, "uusdc")],
        )
        .is_err(),
        "the passed in funds have a denom that's not required so they dont't match the required funds"
    );

    assert!(funds_match_balance(
        &required_tokens,
        &vec![coin(100, "ubtc"), coin(1_000_000_000, "uusdc")],
    )
    .is_ok(),);
}

#[test]
fn required_incentives_tokens_test() {
    let incentives = vec![
        (
            IncentiveCriteria::AllDonationsAbove {
                min_donation: 5_000_000u64.into(),
            },
            Reward::NftAirdrop,
        ),
        (
            IncentiveCriteria::AllDonationsAbove {
                min_donation: 0u64.into(),
            },
            Reward::TokenDistribution {
                token_to_airdrop: coin(100, "ubtc"),
                distribution_type: TokenAirdropDistributionType::Proportional,
            },
        ),
        (
            IncentiveCriteria::PercentTokensDonated {
                from_percent: Decimal::zero(),
                to_percent: Decimal::percent(50),
            },
            Reward::TokenDistribution {
                token_to_airdrop: coin(1_000_000_000, "uusdc"),
                distribution_type: TokenAirdropDistributionType::Equal,
            },
        ),
        (
            IncentiveCriteria::FirstDonors {
                count: 10,
                min_donation: 1_000_000u64.into(),
            },
            Reward::TokenDistribution {
                token_to_airdrop: coin(1000, "ubtc"),
                distribution_type: TokenAirdropDistributionType::Equal,
            },
        ),
    ];

    let balance = required_incentives_tokens(&incentives);

    assert_eq!(
        balance.into_vec(),
        vec![coin(1_100, "ubtc"), coin(1_000_000_000, "uusdc")]
    );
}
