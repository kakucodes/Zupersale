"use client";

import { Box, InputLabel, Stack, Typography } from "@mui/material";
import { DateTimePicker } from "@mui/x-date-pickers/DateTimePicker";
import dayjs from "dayjs";
import React from "react";
import { Controller } from "react-hook-form";

type InputTextProps = {
  name: string;
  label: string;
  control: any;
};

const FormInputDate = ({ name, label, control }: InputTextProps) => {
  return (
    <Controller
      name={name}
      control={control}
      render={({
        field: { onChange, value },
        fieldState: { error },
        formState,
      }) => (
        <>
          <Stack sx={{ display: "flex", flexDirection: "column" }}>
            <Box paddingBottom={2}>
              <Typography>{label}*</Typography>
            </Box>
            <DateTimePicker
              disablePast
              onChange={onChange}
              value={value}
              slotProps={{
                textField: {
                  required: true,
                },
              }}
            />
          </Stack>
        </>
      )}
    />
  );
};

export default FormInputDate;
