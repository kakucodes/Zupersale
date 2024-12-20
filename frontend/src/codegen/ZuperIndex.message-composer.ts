/**
* This file was automatically generated by @cosmwasm/ts-codegen@1.11.1.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { MsgExecuteContractEncodeObject } from "@cosmjs/cosmwasm-stargate";
import { MsgExecuteContract } from "cosmjs-types/cosmwasm/wasm/v1/tx";
import { toUtf8 } from "@cosmjs/encoding";
import { Decimal, Uint128, InstantiateMsg, Coin, ExecuteMsg, Uint64, Binary, Timestamp, IncentiveCriteria, Reward, TokenAirdropDistributionType, CampaignMetadata, QueryMsg, CampaignStatus, CampaignType, Addr, ArrayOfCampaignsResponse, CampaignsResponse, ValidatedCampaignMetadata, ArrayOfWhitelistEntry, WhitelistEntry, IndexSettings } from "./ZuperIndex.types";
export interface ZuperIndexMsg {
  contractAddress: string;
  sender: string;
  migrateCampaigns: ({
    codeId,
    limit,
    msg,
    startAfter
  }: {
    codeId: Uint64;
    limit?: number;
    msg: Binary;
    startAfter?: string;
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  enforceWhitelist: (_funds?: Coin[]) => MsgExecuteContractEncodeObject;
  grantWhitelist: ({
    address,
    testCreationOnly
  }: {
    address: string;
    testCreationOnly: boolean;
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  revokeWhitelist: (_funds?: Coin[]) => MsgExecuteContractEncodeObject;
  setAdmin: (_funds?: Coin[]) => MsgExecuteContractEncodeObject;
  setCampaignContractCodeId: (_funds?: Coin[]) => MsgExecuteContractEncodeObject;
  createCampaign: ({
    campaignMetadata
  }: {
    campaignMetadata: CampaignMetadata;
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  deleteCampaign: ({
    address
  }: {
    address: string;
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
}
export class ZuperIndexMsgComposer implements ZuperIndexMsg {
  sender: string;
  contractAddress: string;
  constructor(sender: string, contractAddress: string) {
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.migrateCampaigns = this.migrateCampaigns.bind(this);
    this.enforceWhitelist = this.enforceWhitelist.bind(this);
    this.grantWhitelist = this.grantWhitelist.bind(this);
    this.revokeWhitelist = this.revokeWhitelist.bind(this);
    this.setAdmin = this.setAdmin.bind(this);
    this.setCampaignContractCodeId = this.setCampaignContractCodeId.bind(this);
    this.createCampaign = this.createCampaign.bind(this);
    this.deleteCampaign = this.deleteCampaign.bind(this);
  }
  migrateCampaigns = ({
    codeId,
    limit,
    msg,
    startAfter
  }: {
    codeId: Uint64;
    limit?: number;
    msg: Binary;
    startAfter?: string;
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          migrate_campaigns: {
            code_id: codeId,
            limit,
            msg,
            start_after: startAfter
          }
        })),
        funds: _funds
      })
    };
  };
  enforceWhitelist = (_funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          enforce_whitelist: {}
        })),
        funds: _funds
      })
    };
  };
  grantWhitelist = ({
    address,
    testCreationOnly
  }: {
    address: string;
    testCreationOnly: boolean;
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          grant_whitelist: {
            address,
            test_creation_only: testCreationOnly
          }
        })),
        funds: _funds
      })
    };
  };
  revokeWhitelist = (_funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          revoke_whitelist: {}
        })),
        funds: _funds
      })
    };
  };
  setAdmin = (_funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          set_admin: {}
        })),
        funds: _funds
      })
    };
  };
  setCampaignContractCodeId = (_funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          set_campaign_contract_code_id: {}
        })),
        funds: _funds
      })
    };
  };
  createCampaign = ({
    campaignMetadata
  }: {
    campaignMetadata: CampaignMetadata;
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          create_campaign: {
            campaign_metadata: campaignMetadata
          }
        })),
        funds: _funds
      })
    };
  };
  deleteCampaign = ({
    address
  }: {
    address: string;
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          delete_campaign: {
            address
          }
        })),
        funds: _funds
      })
    };
  };
}