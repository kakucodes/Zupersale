"use client";

import React, { FC, PropsWithChildren } from "react";
import {
  Stack,
  Button,
  Box,
  Unstable_Grid2 as Grid,
  Typography,
  Toolbar,
} from "@mui/material";
import { useRouter } from 'next/navigation'
import { WalletConnect } from "../WalletConnect/WalletConnect";
import CreateNft from "../button/CreateNft";

const Layout: FC<PropsWithChildren> = ({ children }) => {
  const router = useRouter()
  return (
    <Box sx={{ position: "relative", padding: 0 }}>
      <Stack component="nav" py={1} px={{ xs: 2, md: 5 }}>
        <Grid
          container
          xl={9}
          justifyContent="space-between"
          sx={{ alignSelf: { xl: "center" } }}
        >
          <Grid alignItems="center">
            <Button
              onClick={() => {
                router.push("/");
              }}
              sx={{ cursor: "pointer", paddingLeft: 0 }}
            >
              <Typography
                sx={{ paddingLeft: 1 }}
                fontSize={20}
                fontWeight={520}
                color="primary"
              >
                Zuper
              </Typography>
            </Button>
          </Grid>
          <Grid alignContent="center"></Grid>
          <Grid
            display="flex"
            gap={1}
            justifyContent="flex-end"
            alignItems="center"
          > 
            <CreateNft/>
            <WalletConnect />
          </Grid>
        </Grid>
      </Stack> 
      <Stack component="main" direction="column" minHeight="calc(100vh - 128px)">
      {children}
      </Stack>
      <Stack
          component="footer"
          position="fixed"
          sx={{ position: 'relative', bottom: 0 }}
        >
           <Toolbar sx={{ flex: 1, direction: 'row', justifyContent: 'center' }}>            
           </Toolbar>
        </Stack>
    </Box>
  );
};

export default Layout;
