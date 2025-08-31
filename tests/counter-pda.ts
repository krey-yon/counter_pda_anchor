import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CounterPda } from "../target/types/counter_pda";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("counter-pda", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.counterPda as Program<CounterPda>;
  const provider = anchor.AnchorProvider.local();

  it("counter initialized!", async () => {
    let [counter] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("counter1"), provider.publicKey.toBytes()],
      program.programId
    );
    console.log("your program address is :", counter.toString());

    // Add your test here.
    // const tx = await program.methods.initialize().rpc();
    const tx = await program.methods
      .createCounter()
      .accounts({
        authority: provider.publicKey,
        counter: counter,
        systemProgram: SYSTEM_PROGRAM_ID,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Counter updated", async () => {
    let [counter_pubkey] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("counter1"), provider.publicKey.toBytes()],
      program.programId
    );
    const counter = await program.account.counter.fetch(counter_pubkey);
    console.log("current counter is: ", counter.count.toNumber());
    const tx = await program.methods
      .updateCounter()
      .accounts({
        authority: provider.publicKey,
        counter: counter_pubkey,
      })
      .rpc();

    console.log("txn", tx);
    const updated_counter = await program.account.counter.fetch(counter_pubkey);
    console.log("now the counter is:", updated_counter.count.toNumber());
  });
});
