"use client";

import {
  Box,
  Button,
  ButtonGroup,
  Card,
  CardContent,
  Dialog,
  DialogContent,
  Stack,
  TextField,
  ToggleButton,
  ToggleButtonGroup,
  Typography,
} from "@mui/material";
import React, { useState } from "react";
import { Controller } from "react-hook-form";

type IncentiveFormProps = {
  name: string;
  label: string;
  control: any;
};

const CriteriaButtons = [
  <ToggleButton value={"three"} key="one">
    First Donations
  </ToggleButton>,
  <ToggleButton value={"three"} key="two">
    Biggest Donations
  </ToggleButton>,
  <ToggleButton value={"three"} key="three">
    Percent Donations
  </ToggleButton>,
];

const RewardButtons = [
  <ToggleButton value={"three"} key="one">
    1 of 1 NFT
  </ToggleButton>,
  <ToggleButton value={"three"} key="two">
    NFT Airdrop
  </ToggleButton>,
  <ToggleButton value={"three"} key="three">
    Whitelist
  </ToggleButton>,
  <ToggleButton value={"three"} key="three">
    Token
  </ToggleButton>,
];
const IncentiveForm = ({ name, label, control }: IncentiveFormProps) => {
  const [openIncentiveModal, setOpenIncentiveModal] = useState(false);
  return (
    <Controller
      name={name}
      control={control}
      render={({
        field: { onChange, value },
        fieldState: { error },
        formState,
      }) => (
        <Stack sx={{ display: "flex", flexDirection: "column" }}>
          <Box paddingBottom={2} display="flex" justifyContent="space-between">
            <Typography>{label}*</Typography>
            <Button onClick={() => setOpenIncentiveModal(true)}>+ Add</Button>
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
                  <Stack>
                    <Typography variant="h6">Add Incentive</Typography>
                    <Typography variant="body2">
                      Choose your desired incentive criteria and its reward here
                      and edit in the next step.
                    </Typography>
                  </Stack>
                  <Box
                    display="flex"
                    flexDirection="row"
                    justifyContent="space-evenly"
                  >
                    <Stack gap={2}>
                      <Typography>Choose incentive criteria</Typography>
                      <ToggleButtonGroup orientation="vertical">
                        {CriteriaButtons}
                      </ToggleButtonGroup>
                    </Stack>
                    <Typography alignContent="center">Receives</Typography>
                    <Stack gap={2}>
                      <Typography>Choose reward option</Typography>
                      <ToggleButtonGroup orientation="vertical">
                        {RewardButtons}
                      </ToggleButtonGroup>
                    </Stack>
                  </Box>
                  <Button>Add</Button>
                </Stack>
              </DialogContent>
            </Dialog>
          </Box>
          <Box>
            <Card>
              <CardContent>
                <Typography>No Incentives</Typography>
              </CardContent>
            </Card>
          </Box>
        </Stack>
      )}
    />
  );
};

export default IncentiveForm;
