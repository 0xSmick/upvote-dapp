const {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  Transaction,
  sendAndConfirmTransaction,
} = require("@solana/web3.js");

const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;

const main = async () => {
  console.log("🚀 Starting test...");
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Upvote;

  const baseAccount = anchor.web3.Keypair.generate();

  const tx = await program.rpc.initialize({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log("📝 Your transaction signature", tx);

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("Submission Count", account.totalSubmissions.toString());

  await program.rpc.addSubmission(
    "this is the title",
    "this is the description",
    {
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
      },
    }
  );

  console.log("Added Submission");
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("Total Submissions", account.totalSubmissions.toString());
  console.log("Submission", account.submissionList[0]);
};

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();
