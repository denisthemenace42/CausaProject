import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CausaVault } from "../target/types/project";

describe("project", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.CausaVault as Program<CausaVault>;

  const user = anchor.web3.Keypair.generate();

  it("Initializes the vault", async () => {
    const [vaultPda] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("vault")],
      program.programId
    );

    await program.methods
      .initialize()
      .accounts({
        signer: provider.wallet.publicKey,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Vault initialized at:", vaultPda.toBase58());
  });

  it("Claims rewards", async () => {
    const [vaultPda] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("vault")],
      program.programId
    );

    await program.methods
      .claimReward()
      .accounts({
        user: provider.wallet.publicKey,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Rewards claimed.");
  });
});
