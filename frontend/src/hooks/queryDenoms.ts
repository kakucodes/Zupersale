import { useQueries, useQuery } from "@tanstack/react-query";
import React from "react";
import { SkipClient, SKIP_API_URL } from "@skip-go/client";

export const client = new SkipClient();

// get assets by chain id
export const stargazeDenoms = client.assets({
  chainID: "stargaze-1",
});

export const useQueryAcceptedDenoms = () => {
  return useQuery({
    queryKey: ["denom"],
    queryFn: () =>
      stargazeDenoms
        .then((resp) => resp["stargaze-1"])
        .then((denoms) =>
          denoms.sort((a, b) =>
            (a.recommendedSymbol || a.symbol || "").localeCompare(
              b.recommendedSymbol || b.symbol || ""
            )
          )
        ),
  });
};
