"use client";

import { stargazeDenoms, useQueryAcceptedDenoms } from "@/hooks/queryDenoms";
import {
  Avatar,
  Box,
  Button,
  Checkbox,
  FormControlLabel,
  InputLabel,
  MenuItem,
  Paper,
  Select,
  Stack,
  TextField,
  Typography,
} from "@mui/material";
import React, { useState } from "react";
import { useForm, SubmitHandler } from "react-hook-form";
import FormInputText from "./FormInputText";
import FormSelect from "./FormDropDown";
import FormInputDate from "./FormInputDate";
import FormInputCheckbox from "./FormInputCheckbox";
import IncentiveForm from "./IncentiveForm";

const CampaignForm = () => {
  const {
    register,
    control,
    handleSubmit,
    formState: { errors },
  } = useForm();

  return (
    <Paper elevation={2} sx={{ padding: 4 }}>
      <Box
        sx={{
          display: "flex",
          flexDirection: "column",
          flexWrap: "wrap",
          gap: 2,
          padding: 3,
        }}
      >
        <Typography variant="h6">
          Fill out the form to create the NFT Fund.
        </Typography>
        <form
          onSubmit={handleSubmit((data) => {
            console.log(data, "data");
          })}
        >
          <Box
            sx={{
              display: "flex",
              flexDirection: "column",
              gap: 3,
            }}
          >
            <FormInputText
              name="name"
              label="Campaign Name"
              control={control}
            />
            <FormInputText
              name="address"
              label="Campaign Address"
              control={control}
            />
            <FormInputText
              name="description"
              label="Campaign Description"
              control={control}
              multiline
            />
            <FormInputText
              name="twitter"
              label="Twitter Handle"
              control={control}
            />
            <FormSelect
              name="acceptedDenom"
              label="Accepted Denom"
              control={control}
            />
            <FormInputDate
              name="startDate"
              label="Campaign Start Date/Time"
              control={control}
            />
            <FormInputDate
              name="endDate"
              label="Campaign End Date/Time"
              control={control}
            />
            <IncentiveForm
              rewardName="reward"
              incentiveName="incentive"
              label="Campaign Incentives"
              rewardControl={control}
              incentiveControl={control}
            />
            <FormInputCheckbox
              name="is_nsfw"
              label="Is this NFT campaign contains any harmful contents?"
              control={control}
            />
            <FormInputCheckbox
              name="test_campaign"
              label="Is this NFT campaign a test?"
              control={control}
            />

            <Button type="submit" variant="contained">
              Submit
            </Button>
          </Box>
        </form>
      </Box>
    </Paper>
  );
};

export default CampaignForm;
