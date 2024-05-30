import { Cluster, PublicKey } from "@solana/web3.js";

export const TODO_PROGRAM_ID = new PublicKey(
  "16RGbKergxtWkpfHe21ZqEXk9x3G8tuTb2X8dT42Ge8"
);

export function getProgramId(cluster: Cluster) {
  switch (cluster) {
    case "devnet":
    case "testnet":
    case "mainnet-beta":
    default:
      return TODO_PROGRAM_ID;
  }
}
