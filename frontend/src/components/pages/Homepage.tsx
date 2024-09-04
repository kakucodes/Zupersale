"use client";

import { Box, Divider, Stack, styled, Typography } from "@mui/material";

const TextContainer = styled(Box)({
  display: "flex",
  flexDirection: "column",
  alignContent: "center",
  padding: 2,
});

const Homepage = () => {
  return (    
      <Stack
        sx={{        
          display: "flex",
          flexDirection: "row",
          justifyContent: "center",
          gap: 2,
        }}
      >
        <TextContainer>
          <Typography variant="h4" gutterBottom>
            Project Funded
          </Typography>
          <Typography variant="body1" alignSelf="center">
            34
          </Typography>
        </TextContainer>
        <Divider orientation="vertical" flexItem />
        <TextContainer>
          <Typography variant="h4" gutterBottom>
            Total Raised
          </Typography>
          <Typography variant="body1" alignSelf="center">
            $19,034
          </Typography>
        </TextContainer>
        <Divider orientation="vertical" flexItem />
        <TextContainer>
          <Typography variant="h4" gutterBottom>
            Contribution Made
          </Typography>
          <Typography variant="body1" alignSelf="center">
            1,254
          </Typography>
        </TextContainer>
      </Stack>
   
  );
};

export default Homepage;
