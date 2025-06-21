import * as Client from "../../packages/counter";
import { RPC_URL } from "../config";

export const counter = new Client.Client({
  ...Client.networks.testnet,
  rpcUrl: RPC_URL,
});
