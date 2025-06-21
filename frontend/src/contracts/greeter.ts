import * as Client from "../../packages/greeter";
import { RPC_URL } from "../config";

export const greeter = new Client.Client({
  ...Client.networks.testnet,
  rpcUrl: RPC_URL,
});
