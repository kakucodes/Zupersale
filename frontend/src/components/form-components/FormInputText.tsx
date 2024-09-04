"use client";

import { Box, TextField } from "@mui/material";
import React from "react";
import { Controller, useFormContext } from "react-hook-form";

type InputTextProps = {
  name: string;
  label: string;
  control: any;
  multiline?: boolean;
};

const FormInputText = ({ name, label, control, multiline }: InputTextProps) => {
  return (
    <Controller
      name={name}
      control={control}
      render={({
        field: { onChange, value },
        fieldState: { error },
        formState,
      }) => (
        <TextField
          fullWidth
          required
          multiline
          rows={multiline ? 4 : 1}
          onChange={onChange}
          error={!!error}
          label={label}
          placeholder={label}
          value={value}
        />
      )}
    />
  );
};

export default FormInputText;
