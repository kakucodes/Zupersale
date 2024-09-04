"use client";

import { Button } from "@mui/material";
import { getAvailableWallets, useConnect, useDisconnect, WalletType } from "graz";

export const WalletConnect = () => {
  const { connect } = useConnect();
  const { disconnect } = useDisconnect({
    onSuccess: () => console.log("wallet disconnected"),
  });

  const handleConnect = (wallet: WalletType) => {
    connect({ walletType: wallet, chainId: "stargaze-1" });
  };
  const wallets = getAvailableWallets();
  console.log({wallets, handleConnect, disconnect,connect}, "wallet")

  return (
    <Button
      variant="contained"
      onClick={() => handleConnect(WalletType.KEPLR)}
    >
      Connect
    </Button>
  );
};

export default WalletConnect;
