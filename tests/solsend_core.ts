import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolsendCore } from "../target/types/solsend_core";

describe("solsend_core", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.SolsendCore as Program<SolsendCore>;

  const creator = provider.wallet.publicKey
  const channel = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('Channel'), creator.toBuffer()],
    program.programId
  )[0]
  const notification = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('Notification'), creator.toBuffer()],
    program.programId
  )[0]

  it("Creates Channel!", async () => {
    // Add your test here.
    const tx = await program.methods.initChannel("ENS", "some description", "icon url").accounts({ creator, channel }).rpc();
    const channelData = await program.account.channel.fetch(channel)
    console.log(channelData)
    console.log("Your transaction signature", tx);
  });

  // TODO: Note to run this test you must update the `space` for notification account to 1024.
  it("Sends notifications!", async () => {
    const tx = await program.methods.publishNotification("New Name Created", "A new name has been registered yadda xdfbseb sbserb earbawrbyadda yadda").accounts({ sender: creator, channel, notification }).rpc();
    const notifData = await program.account.notification.fetch(notification)
    console.log(notifData)
    console.log("Your transaction signature", tx);
  })
});
