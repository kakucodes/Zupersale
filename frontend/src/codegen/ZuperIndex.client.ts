/**
* This file was automatically generated by @cosmwasm/ts-codegen@1.11.1.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import { Decimal, Uint128, InstantiateMsg, Coin, ExecuteMsg, Uint64, Binary, Timestamp, IncentiveCriteria, Reward, CampaignMetadata, QueryMsg, CampaignStatus, CampaignType, Addr, ArrayOfCampaignsResponse, CampaignsResponse, ValidatedCampaignMetadata, ArrayOfWhitelistEntry, WhitelistEntry, IndexSettings } from "./ZuperIndex.types";
export interface ZuperIndexReadOnlyInterface {
  contractAddress: string;
  creatorWhitelist: ({
    limit,
    startAfter
  }: {
    limit?: number;
    startAfter?: string;
  }) => Promise<ArrayOfWhitelistEntry>;
  campaigns: ({
    campaignStatus,
    campaignType,
    limit,
    startAfter
  }: {
    campaignStatus?: CampaignStatus;
    campaignType: CampaignType;
    limit?: number;
    startAfter?: string;
  }) => Promise<ArrayOfCampaignsResponse>;
}
export class ZuperIndexQueryClient implements ZuperIndexReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;
  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.creatorWhitelist = this.creatorWhitelist.bind(this);
    this.campaigns = this.campaigns.bind(this);
  }
  creatorWhitelist = async ({
    limit,
    startAfter
  }: {
    limit?: number;
    startAfter?: string;
  }): Promise<ArrayOfWhitelistEntry> => {
    return this.client.queryContractSmart(this.contractAddress, {
      creator_whitelist: {
        limit,
        start_after: startAfter
      }
    });
  };
  campaigns = async ({
    campaignStatus,
    campaignType,
    limit,
    startAfter
  }: {
    campaignStatus?: CampaignStatus;
    campaignType: CampaignType;
    limit?: number;
    startAfter?: string;
  }): Promise<ArrayOfCampaignsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      campaigns: {
        campaign_status: campaignStatus,
        campaign_type: campaignType,
        limit,
        start_after: startAfter
      }
    });
  };
}
export interface ZuperIndexInterface extends ZuperIndexReadOnlyInterface {
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
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  enforceWhitelist: (fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  grantWhitelist: ({
    address,
    testCreationOnly
  }: {
    address: string;
    testCreationOnly: boolean;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  revokeWhitelist: (fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  setAdmin: (fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  setCampaignContractCodeId: (fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  createCampaign: ({
    campaignMetadata
  }: {
    campaignMetadata: CampaignMetadata;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  deleteCampaign: ({
    address
  }: {
    address: string;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
}
export class ZuperIndexClient extends ZuperIndexQueryClient implements ZuperIndexInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;
  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
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
  migrateCampaigns = async ({
    codeId,
    limit,
    msg,
    startAfter
  }: {
    codeId: Uint64;
    limit?: number;
    msg: Binary;
    startAfter?: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      migrate_campaigns: {
        code_id: codeId,
        limit,
        msg,
        start_after: startAfter
      }
    }, fee, memo, _funds);
  };
  enforceWhitelist = async (fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      enforce_whitelist: {}
    }, fee, memo, _funds);
  };
  grantWhitelist = async ({
    address,
    testCreationOnly
  }: {
    address: string;
    testCreationOnly: boolean;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      grant_whitelist: {
        address,
        test_creation_only: testCreationOnly
      }
    }, fee, memo, _funds);
  };
  revokeWhitelist = async (fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      revoke_whitelist: {}
    }, fee, memo, _funds);
  };
  setAdmin = async (fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      set_admin: {}
    }, fee, memo, _funds);
  };
  setCampaignContractCodeId = async (fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      set_campaign_contract_code_id: {}
    }, fee, memo, _funds);
  };
  createCampaign = async ({
    campaignMetadata
  }: {
    campaignMetadata: CampaignMetadata;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      create_campaign: {
        campaign_metadata: campaignMetadata
      }
    }, fee, memo, _funds);
  };
  deleteCampaign = async ({
    address
  }: {
    address: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      delete_campaign: {
        address
      }
    }, fee, memo, _funds);
  };
}