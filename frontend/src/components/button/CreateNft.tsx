"use client";

import { Button } from '@mui/material'
import { useRouter } from 'next/navigation';
import React from 'react'

const CreateNft = () => {

  const router = useRouter()
  return (
    <Button
      variant="contained"
      onClick={() => router.push("/createNFT") }
    >
      Create NFT
    </Button>
  )
}

export default CreateNft