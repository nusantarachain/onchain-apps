import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { ContractPromise } from "@polkadot/api-contract";
import { cryptoWaitReady } from "@polkadot/util-crypto";

import { web3Accounts, web3Enable, web3FromAddress } from "@polkadot/extension-dapp";

const abiJson = require("~/assets/umkm4.json");

const keyring = new Keyring({ type: "sr25519", ss58Format: 42 });

async function ConnectNodeAndContract(contractAddress) {
  await cryptoWaitReady();

  const provider = new WsProvider("wss://satnet-testnet.node.nuchain.network");
//   const provider = new WsProvider("wss://id.node.nuchain.network");
  const api = await ApiPromise.create({
    provider,
    types: {
      Address: "MultiAddress",
      LookupSource: "MultiAddress"
    }
  });

  const contract = new ContractPromise(api, abiJson, contractAddress);

  const [chain, nodeName, nodeVersion] = await Promise.all([
    api.rpc.system.chain(),
    api.rpc.system.name(),
    api.rpc.system.version()
  ]);

  console.log(
    `You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`
  );

  return contract;
}

var allAccounts = [];

/**
 * Fungsi untuk menginisialisasi web3
 * ini akan mengakses data akun di extension yang provide web3 keystore.
 */
async function init() {
  const allInjected = await web3Enable("UMKM4");
  allAccounts = await web3Accounts();
}

function getInjectedAccounts() {
  return allAccounts;
}

/**
 * Gunakan akun dari web3 keystore.
 * Ini akan membangkitkan extension (apabila ada) untuk konfirmasi akses ke pengguna.
 */
async function useAccount(address){
    return await web3FromAddress(address); 
}

export default (context, inject) => {
  inject("nuchain", {
    init,
    getInjectedAccounts,
    useAccount,
    ConnectNodeAndContract,
    keyring
  });
};
