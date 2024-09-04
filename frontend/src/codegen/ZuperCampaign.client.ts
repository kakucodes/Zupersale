/**
* This file was automatically generated by @cosmwasm/ts-codegen@1.11.1.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import { Timestamp, Uint64, IncentiveCriteria, Decimal, Reward, InstantiateMsg, CampaignMetadata, ProtocolInstantiateSettings, ExecuteMsg, QueryMsg, Addr, ValidatedCampaignMetadata, CampaignStatus, Uint128, ArrayOfTupleOfAddrAndCoin, Coin, ArrayOfTupleOfAddrAndCoinAndTimestamp, UserDonationsResponse } from "./ZuperCampaign.types";
export interface ZuperCampaignReadOnlyInterface {
  contractAddress: string;
  donationsBySize: ({
    limit,
    startAfter
  }: {
    limit?: number;
    startAfter?: string;
  }) => Promise<ArrayOfTupleOfAddrAndCoin>;
  donationsByTime: ({
    ascending,
    limit,
    startAfter
  }: {
    ascending: boolean;
    limit?: number;
    startAfter?: Timestamp;
  }) => Promise<ArrayOfTupleOfAddrAndCoinAndTimestamp>;
  userDonation: ({
    address
  }: {
    address: string;
  }) => Promise<UserDonationsResponse>;
}
export class ZuperCampaignQueryClient implements ZuperCampaignReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;
  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.donationsBySize = this.donationsBySize.bind(this);
    this.donationsByTime = this.donationsByTime.bind(this);
    this.userDonation = this.userDonation.bind(this);
  }
  donationsBySize = async ({
    limit,
    startAfter
  }: {
    limit?: number;
    startAfter?: string;
  }): Promise<ArrayOfTupleOfAddrAndCoin> => {
    return this.client.queryContractSmart(this.contractAddress, {
      donations_by_size: {
        limit,
        start_after: startAfter
      }
    });
  };
  donationsByTime = async ({
    ascending,
    limit,
    startAfter
  }: {
    ascending: boolean;
    limit?: number;
    startAfter?: Timestamp;
  }): Promise<ArrayOfTupleOfAddrAndCoinAndTimestamp> => {
    return this.client.queryContractSmart(this.contractAddress, {
      donations_by_time: {
        ascending,
        limit,
        start_after: startAfter
      }
    });
  };
  userDonation = async ({
    address
  }: {
    address: string;
  }): Promise<UserDonationsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      user_donation: {
        address
      }
    });
  };
}
export interface ZuperCampaignInterface extends ZuperCampaignReadOnlyInterface {
  contractAddress: string;
  sender: string;
  setOwner: ({
    owner
  }: {
    owner: string;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  deposit: ({
    donorAddress
  }: {
    donorAddress: string;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  completeCampaign: (fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  distributeRewards: ({
    limit
  }: {
    limit?: number;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
}
export class ZuperCampaignClient extends ZuperCampaignQueryClient implements ZuperCampaignInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;
  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.setOwner = this.setOwner.bind(this);
    this.deposit = this.deposit.bind(this);
    this.completeCampaign = this.completeCampaign.bind(this);
    this.distributeRewards = this.distributeRewards.bind(this);
  }
  setOwner = async ({
    owner
  }: {
    owner: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      set_owner: {
        owner
      }
    }, fee, memo, _funds);
  };
  deposit = async ({
    donorAddress
  }: {
    donorAddress: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      deposit: {
        donor_address: donorAddress
      }
    }, fee, memo, _funds);
  };
  completeCampaign = async (fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      complete_campaign: {}
    }, fee, memo, _funds);
  };
  distributeRewards = async ({
    limit
  }: {
    limit?: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      distribute_rewards: {
        limit
      }
    }, fee, memo, _funds);
  };
}