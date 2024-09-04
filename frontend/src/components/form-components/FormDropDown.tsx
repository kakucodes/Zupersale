"use client";

import { useQueryAcceptedDenoms } from "@/hooks/queryDenoms";
import {
  Avatar,
  Box,
  FormControl,
  FormControlLabel,
  InputLabel,
  MenuItem,
  Select,
  Stack,
  Switch,
  TextField,
  Typography,
} from "@mui/material";
import React, { useState } from "react";
import { Controller } from "react-hook-form";

type DropdownProps = {
  name: string;
  label: string;
  control: any;
};

const FormSelect = ({ name, label, control }: DropdownProps) => {
  const { data: acceptedDenoms } = useQueryAcceptedDenoms();
  const [showAll, setShowAll] = useState(false);

  const filteredOptions = showAll
    ? acceptedDenoms
    : acceptedDenoms?.filter(({ recommendedSymbol }) =>
        ["USDC", "stATOM", "ATOM", "stSTARS", "STARS"].includes(
          recommendedSymbol || ""
        )
      );

  const acceptedDenomOptions = () => {
    return filteredOptions?.map(({ denom, logoURI, recommendedSymbol }) => {
      return (
        <MenuItem key={denom} value={denom}>
          <Box display="flex" flexDirection="row" alignItems="center">
            <Avatar
              alt="denom img"
              sx={{ width: 26, height: 26, mr: 1 }}
              src={logoURI}
            />
            <Typography>{recommendedSymbol}</Typography>
          </Box>
        </MenuItem>
      );
    });
  };

  return (
    <FormControl fullWidth>
      <Stack
        display="flex"
        flexDirection="row"
        alignItems="center"
        justifyContent="space-between"
      >
        <Typography>{label}*</Typography>
        <FormControlLabel
          control={
            <Switch checked={showAll} onChange={() => setShowAll(!showAll)} />
          }
          label="Show All"
          labelPlacement="end"
        />
      </Stack>
      <Controller
        render={({ field: { onChange, value } }) => (
          <TextField
            select
            fullWidth
            required
            id="acceptedDenom"
            onChange={onChange}
            value={value}
          >
            {acceptedDenomOptions()}
          </TextField>
        )}
        control={control}
        name={name}
      />
    </FormControl>
  );
};

export default FormSelect;
