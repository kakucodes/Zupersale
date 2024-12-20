/**
* This file was automatically generated by @cosmwasm/ts-codegen@1.11.1.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { MsgExecuteContractEncodeObject } from "@cosmjs/cosmwasm-stargate";
import { MsgExecuteContract } from "cosmjs-types/cosmwasm/wasm/v1/tx";
import { toUtf8 } from "@cosmjs/encoding";
import { Timestamp, Uint64, IncentiveCriteria, Decimal, Reward, TokenAirdropDistributionType, Uint128, InstantiateMsg, CampaignMetadata, Coin, ProtocolInstantiateSettings, ExecuteMsg, QueryMsg, Addr, ValidatedCampaignMetadata, CampaignStatus, ArrayOfTupleOfAddrAndCoin, ArrayOfTupleOfAddrAndCoinAndTimestamp, UserDonationsResponse } from "./ZuperCampaign.types";
export interface ZuperCampaignMsg {
  contractAddress: string;
  sender: string;
  setOwner: ({
    owner
  }: {
    owner: string;
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  deposit: ({
    donorAddress
  }: {
    donorAddress: string;
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  completeCampaign: (_funds?: Coin[]) => MsgExecuteContractEncodeObject;
  distributeRewards: ({
    limit
  }: {
    limit?: number;
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
}
export class ZuperCampaignMsgComposer implements ZuperCampaignMsg {
  sender: string;
  contractAddress: string;
  constructor(sender: string, contractAddress: string) {
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.setOwner = this.setOwner.bind(this);
    this.deposit = this.deposit.bind(this);
    this.completeCampaign = this.completeCampaign.bind(this);
    this.distributeRewards = this.distributeRewards.bind(this);
  }
  setOwner = ({
    owner
  }: {
    owner: string;
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          set_owner: {
            owner
          }
        })),
        funds: _funds
      })
    };
  };
  deposit = ({
    donorAddress
  }: {
    donorAddress: string;
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          deposit: {
            donor_address: donorAddress
          }
        })),
        funds: _funds
      })
    };
  };
  completeCampaign = (_funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          complete_campaign: {}
        })),
        funds: _funds
      })
    };
  };
  distributeRewards = ({
    limit
  }: {
    limit?: number;
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          distribute_rewards: {
            limit
          }
        })),
        funds: _funds
      })
    };
  };
}