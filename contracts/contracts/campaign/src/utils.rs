use cosmwasm_std::{coin, Coin};
use cw_utils::NativeBalance;

use crate::{
    msg::{Incentives, Reward},
    ContractError,
};

pub fn required_incentives_tokens(incentives: &Incentives) -> NativeBalance {
    let balance = incentives
        .iter()
        .filter_map(|(incentive, reward)| match reward {
            Reward::TokenDistribution {
                token_to_airdrop,
                distribution_type,
            } => Some(token_to_airdrop),
            _ => None,
        })
        .fold(NativeBalance::default(), |bal, token| bal + token.clone());

    balance
}

pub fn funds_match_balance(balance: &NativeBalance, funds: &[Coin]) -> Result<(), ContractError> {
    // let balance_vec = balance.clone().into_vec().into_iter();
    // funds.iter().try_for_each(|fund| {
    //     match balance_vec
    //         .clone()
    //         .find(|balance_coin| balance_coin.eq(fund))
    //     {
    //         Some(expected) if fund.amount.lt(&expected.amount) => {
    //             Err(ContractError::FundsMissmatch {
    //                 expected: expected.clone(),
    //                 received: fund.clone(),
    //             })
    //         }
    //         None => Err(ContractError::UnexpectedDenom(fund.denom.clone())),
    //         Some(expected) => Ok(()),
    //     }
    // })
    let funds_iter = funds.iter();

    balance.clone().into_vec().iter().try_for_each(|expected| {
        match funds_iter.clone().find(|fund| fund.eq(&expected)) {
            Some(sent_fund) if sent_fund.amount.clone().lt(&expected.amount) => {
                Err(ContractError::FundsMissmatch {
                    expected: expected.clone(),
                    received: sent_fund.clone(),
                })
            }
            None => Err(ContractError::FundsMissmatch {
                expected: expected.clone(),
                received: coin(0, expected.denom.clone()),
            }),
            Some(_) => Ok(()),
        }
    })
}
