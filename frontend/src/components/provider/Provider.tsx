'use client'

import { GrazProvider, WalletType,  } from 'graz'
import { stargaze } from 'graz/chains'
import { QueryClientProvider, QueryClient } from '@tanstack/react-query'
import type { FC, PropsWithChildren } from 'react'
import { LocalizationProvider } from '@mui/x-date-pickers/LocalizationProvider'
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs'

const Provider: FC<PropsWithChildren> = ({ children }) => {   
   
const queryClient = new QueryClient()

  return (
    <QueryClientProvider client={queryClient}>

    <LocalizationProvider dateAdapter={AdapterDayjs}>
      <GrazProvider grazOptions={{chains: [stargaze], defaultWallet: WalletType.KEPLR,}}>
        {children}
      </GrazProvider>
    </LocalizationProvider>
    </QueryClientProvider>
  )
}

export default Provider