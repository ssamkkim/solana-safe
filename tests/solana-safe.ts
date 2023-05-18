import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import * as assert from "assert";
import * as bs58 from "bs58";
import { SolanaSafe } from "../target/types/solana_safe";

describe("solana-safe", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SolanaSafe;

  it("can initialize a safe", async () => {
    const safe = anchor.web3.Keypair.generate();
    await program.rpc.initializeSafe('test safe', {
      accounts: {
        safe: safe.publicKey,
        owner: program.provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [
        safe,
      ],
    });

    const safeAccount = await program.account.safe.fetch(safe.publicKey);

    assert.equal(safeAccount.owner.toBase58(), program.provider.wallet.publicKey.toBase58());
    assert.equal(safeAccount.name, "test safe");
    assert.equal(safeAccount.balance, 0);
  });
});
