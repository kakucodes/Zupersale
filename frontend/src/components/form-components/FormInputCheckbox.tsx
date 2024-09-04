"use client";

import {
  Checkbox,
  FormControl,
  FormControlLabel,
  FormLabel,
} from "@mui/material";
import React from "react";
import { Controller } from "react-hook-form";

type InputCheckboxProps = {
  label: string;
  name: string;
  control: any;
  setValue?: any;
};

// const options = [
//   {
//     label: "Is this NFT campaign contains any harmful contents?",
//     value: "1",
//   },
//   {
//     label: "Is this NFT campaign a test?",
//     value: "2",
//   },
// ];

const FormInputCheckbox = ({
  label,
  name,
  control,
  setValue,
}: InputCheckboxProps) => {
  return (
    <FormControl variant={"outlined"}>
      <FormControlLabel
        control={
          <Controller
            name={name}
            render={({
              field: { onChange, value },
              fieldState: { error },
              formState,
            }) => {
              return (
                <Checkbox
                  onChange={onChange}
                  value={value}
                  // checked={selectedItems.includes(option.value)}
                  // onChange={() => handleSelect(option.value)}
                />
              );
            }}
            control={control}
          />
        }
        label={label}
      />
    </FormControl>
  );
};

export default FormInputCheckbox;
