// const anchor = require("@coral-xyz/anchor");
// Import statements
import assert from "assert";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
const { SystemProgram } = anchor.web3;

describe("gifs-dapp-containerized", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  const program = anchor.workspace.Gifsoldapp;
  // let baseAccount;

  it("Starts stuff off, get the transaction, and count GIF", async () => {
    const baseAccount = anchor.web3.Keypair.generate();
    try {
      const txSignature = await program.methods.startStuffOff().accounts({
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      }).signers([baseAccount]).rpc();

      console.log("Transaction Signature:", txSignature);
    } catch (error) {
      console.error("error sending transaction:", error);
    }

    let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('GIF Count', account.totalGifs.toString());

    // addGif
    await program.methods.addGif("https://media1.tenor.com/m/gyMnXhvx_lcAAAAC/seriously-jacob-black.gif").accounts({
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    }).rpc();
    account = await program.account.baseAccount.fetch(baseAccount.publicKey)
    console.log('GIF Count', account.totalGifs.toString());
    console.log('GIF List', account.gifList);
  });
});
