import { Writable, writable } from "svelte/store";
import { ethers } from "ethers";
import Web3Modal from "web3modal";
import WalletConnectProvider from "@walletconnect/web3-provider";

export type ENSType = {
    name: string | null;
    avatar: string | null;
};

export interface Signer {
    ens: ENSType;
    provider: ethers.providers.Web3Provider;
    sign: (statement: string) => Promise<string>;
    id: () => string;
};

export const signer: Writable<Signer | false>  = writable(false);
export const posts: Writable<Array<Object>> = writable([]);
export const witnessUrl = "http://localhost:8787";
export interface KeyType {
    pkh?: {
        eip155: {
            address: string;
            chain_id: string;
        };
    },
    web?: string;
}

export const connectSigner = async (): Promise<void> => {
    let ens: ENSType;
    const providerOptions = {
        walletconnect: {
            package: WalletConnectProvider, // required
            options: {
                infuraId: process.env.INFURA_ID // required
            }
        }
    };

    const web3Modal = new Web3Modal({
        cacheProvider: false,
        providerOptions,
    });
    web3Modal.clearCachedProvider()

    const instance = await web3Modal.connect();
    const provider = new ethers.providers.Web3Provider(instance);
    const s = provider.getSigner();

    if (!s) {
        throw new Error("User cancelled connection");
    }

    const ids = await provider.listAccounts();
    if (ids.length <= 0) {
        throw new Error("No ids found in ethereum connection");
    }

    ens = { name: null, avatar: null };
    ens.name = await provider.lookupAddress(ids[0]);
    const network =
        provider.network.name === "homestead"
            ? "mainnet"
            : provider.network.name;

    ens.avatar = ens.name
        ? `https://metadata.ens.domains/${network}/avatar/${ens.name}`
        : null;


    const sign = async (statement: string): Promise<string> => {
        return s.signMessage(statement)
    };

    const id = (): string => ids[0];

    signer.set({ sign, id, provider, ens });
}

export const disconnect = async () => {
    signer.set(false);

    const providerOptions = {
        /* See Provider Options Section */
    };

    const web3Modal = new Web3Modal({
        network: "mainnet",
        cacheProvider: true,
        providerOptions,
    });

    await web3Modal.clearCachedProvider();

    return;
}