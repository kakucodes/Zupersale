"use client";

import { IncentiveCriteria } from "@/codegen/ZuperCampaign.types";
import {
  Box,
  Button,
  Card,
  CardContent,
  Dialog,
  DialogContent,
  FormControl,
  Stack,
  TextField,
  ToggleButton,
  ToggleButtonGroup,
  Typography,
} from "@mui/material";
import React, { useState } from "react";
import { Controller } from "react-hook-form";

type IncentiveFormProps = {
  rewardName: string;
  incentiveName: string;
  label: string;
  rewardControl: any;
  incentiveControl: any;
};

type ArbitraryThing<T, L extends keyof T> = {
  //IncentiveCriteria
  //  L  === all_donations_above: {
  //  Partial ===  min_donation: Uint64;
  // }
  type: L,
  params: Partial<Extract<T,{[ L in keyof T]: any}>>
}

// type IncentiveKeys = keyof IncentiveCriteria

// type Ji = ArbitraryThing<IncentiveCriteria, 'first_donors'>



// type ReducerState = {
  // criteria: ArbitraryThing<IncentiveCriteria>, 
  // reward: any
// }

// const reducerState = {
//   rules: [{
//   criteria: {
//     type: "all_donations_above",
//     params: {
//       min_donations: 1_000_000
//     }
//   },
//   reward: {
//     type: "whitelist_spot",
//     params: {}
//   },
//   isValid: false
// },
// {
//   criteria: {
//     type: "percent_tokens_donated",
//     params: {
//       from_percent: '0', 
//       to_percent: '0.5'
//     }
//   },
//   reward: {
//     type: "token_distribution",
//     params: {
//       distribution_type: "equal",
//       token_to_airdrop: '2stAtom'
//     }
//   },
//   isValid: false 
// }]
// };

const criteriaLabel = [
  {
    label: "All Donations",
    value: "all",
  },
  {
    label: "First Donations",
    value: "first",
  },
  {
    label: "Biggest Donations",
    value: "biggest",
  },
  {
    label: "Percent Donations",
    value: "percent",
  },
];

const RewardLabel = [
  {
    label: "1 of 1 NFT",
    value: "1/1Nft",
  },
  {
    label: "NFT Airdrop",
    value: "airdrop",
  },
  {
    label: "Whitelist",
    value: "whitelist",
  },
  {
    label: "Token",
    value: "token",
  },
];

const reducer = (state, action) => {

}

const IncentiveForm = ({
  rewardName,
  incentiveName,
  label,
  rewardControl,
  incentiveControl,
}: IncentiveFormProps) => {
  const [openIncentiveModal, setOpenIncentiveModal] = useState(false);
  const [updateIncentive, setUpdateIncentive] = useState();
  // const [state, dispatch] = useReducer(reducer, {})

  const incentiveCriteriaButton = criteriaLabel.map(({ label, value }) => {
    return (
      <ToggleButton key={value} value={value}>
        {label}
      </ToggleButton>
    );
  });

  const rewardOptionButton = RewardLabel.map(({ label, value }) => {
    return (
      <ToggleButton key={value} value={value}>
        {label}
      </ToggleButton>
    );
  });

  return (
    <FormControl fullWidth>
      <Stack sx={{ display: "flex", flexDirection: "column" }}>
        <Box paddingBottom={2} display="flex" justifyContent="space-between">
          <Typography>{label}*</Typography>
          <Button
            variant="outlined"
            onClick={() => setOpenIncentiveModal(true)}
          >
            + Add
          </Button>
        </Box>
      </Stack>
      <Dialog
        fullWidth
        open={openIncentiveModal}
        onClose={() => {
          setOpenIncentiveModal(false);
        }}
        sx={{ backdropFilter: "blur(5px)" }}
      >
        <DialogContent>
          <Stack gap={2}>
            <Box>
              <Typography variant="h6">Add Incentive</Typography>
              <Typography variant="body2">
                Select your preferred incentive criteria and corresponding
                reward here. You can customize the specific reward options in
                the next step.
              </Typography>
            </Box>
            <Box
              display="flex"
              flexDirection="row"
              justifyContent="space-between"
            >
              <Stack gap={2}>
                <Typography>Choose incentive criteria</Typography>
                <Controller
                  name={incentiveName}
                  control={incentiveControl}
                  render={({
                    field: { onChange, value },
                    fieldState: { error },
                    formState,
                  }) => {
                    console.log(value, "incentive values");
                    return (
                      <>
                        <ToggleButtonGroup
                          orientation="vertical"
                          color="primary"
                          onChange={onChange}
                          value={value}
                          exclusive
                        >
                          {incentiveCriteriaButton}
                        </ToggleButtonGroup>
                      </>
                    );
                  }}
                />
              </Stack>
              <Typography alignContent="center">Receives</Typography>
              <Stack gap={2}>
                <Typography>Choose reward option</Typography>
                <Controller
                  name={rewardName}
                  control={rewardControl}
                  render={({
                    field: { onChange, value },
                    fieldState: { error },
                    formState,
                  }) => {
                    console.log(value, "reward value");
                    return (
                      <>
                        <ToggleButtonGroup
                          orientation="vertical"
                          color="primary"
                          onChange={onChange}
                          value={value}
                          exclusive
                        >
                          {rewardOptionButton}
                        </ToggleButtonGroup>
                      </>
                    );
                  }}
                />
              </Stack>
            </Box>
            <Button
              fullWidth
              variant="contained"
              onClick={() => setUpdateIncentive}
            >
              Add
            </Button>
          </Stack>
        </DialogContent>
      </Dialog>
      <Card>
        <CardContent>
          {updateIncentive ? <></> : <Typography>No Incentives Yet</Typography>}
        </CardContent>
      </Card>
    </FormControl>
  );
};

export default IncentiveForm;
