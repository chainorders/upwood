import { detectConcordiumProvider } from "@concordium/browser-wallet-api-helpers";
import { ReceiveMethod } from "../contractClients/GenericContract";
import { AccountAddress, ContractAddress, TransactionHash } from "@concordium/web-sdk";
import concordiumNodeClient from "../contractClients/ConcordiumNodeClient";
import { parseFinalizedUpdate } from "./conversions";

export type TxnStatus = "sending" | "waiting" | "success" | "error" | "none";
export const updateContract = async <T, TOut = never, E = never>(
    account: string,
    contractAddress: string,
    method: ReceiveMethod<T, TOut, E>,
    req: T,
    onStatusUpdate: (status: TxnStatus) => void) => {
    const wallet = await detectConcordiumProvider();
    let walletAccount = await wallet.getMostRecentlySelectedAccount();
    if (!walletAccount || walletAccount != account) {
        const accounts = await wallet.requestAccounts();
        walletAccount = accounts.find((account) => account == account);
    }
    if (!walletAccount) {
        throw "No account selected or account is not the same as the user account";
    }
    const accountAddress = AccountAddress.fromBase58(walletAccount);
    onStatusUpdate("sending");
    const txnHashHex = await method.update(
        wallet,
        accountAddress,
        ContractAddress.create(BigInt(contractAddress)),
        req
    );
    const txnHash = TransactionHash.fromHexString(txnHashHex);
    onStatusUpdate("waiting");
    const outcome = await concordiumNodeClient.waitForTransactionFinalization(txnHash);
    const txnResult = parseFinalizedUpdate(outcome);
    switch (txnResult.tag) {
        case "success": {
            onStatusUpdate("success");
            break;
        }
        case "error": {
            onStatusUpdate("error");
            throw `Failed to update contract: ${txnResult.value.rejectReason}`;
        }
    };

    return txnHash;
};