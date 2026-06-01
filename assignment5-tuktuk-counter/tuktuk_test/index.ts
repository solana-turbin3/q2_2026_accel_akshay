import { AnchorProvider, Wallet, BN } from "@coral-xyz/anchor";
import {
  compileTransaction,
  init,
  queueTask,
} from "@helium/tuktuk-sdk";

import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  TransactionInstruction,
} from "@solana/web3.js";

import yargs from "yargs";
import { hideBin } from "yargs/helpers";

import {
  initializeTaskQueue,
  loadKeypair,
  monitorTask,
} from "./helpers.ts";

async function main() {
  const argv = await yargs(hideBin(process.argv))
    .options({
      queueName: {
        type: "string",
        demandOption: true,
      },
      walletPath: {
        type: "string",
        demandOption: true,
      },
      rpcUrl: {
        type: "string",
        demandOption: true,
      },
      programId: {
        type: "string",
        demandOption: true,
      },
    })
    .argv;

  // ---------------------------------------
  // LOAD WALLET
  // ---------------------------------------

  const keypair: Keypair = loadKeypair(argv.walletPath);

  const connection = new Connection(argv.rpcUrl, "confirmed");

  const wallet = new Wallet(keypair);

  const provider = new AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });

  console.log("wallet:", wallet.publicKey.toBase58());

  // ---------------------------------------
  // INIT TUKTUK
  // ---------------------------------------

  const program = await init(provider);

  // ---------------------------------------
  // INIT TASK QUEUE
  // ---------------------------------------

  const taskQueue = await initializeTaskQueue(
    program,
    argv.queueName
  );

  console.log("task queue:", taskQueue.toBase58());

  // ---------------------------------------
  // YOUR COUNTER PROGRAM
  // ---------------------------------------

  const COUNTER_PROGRAM_ID = new PublicKey(argv.programId);

  // ---------------------------------------
  // FIND PDA
  // ---------------------------------------

  const [counterPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("counter")],
    COUNTER_PROGRAM_ID
  );

  console.log("counter PDA:", counterPda.toBase58());

  // ---------------------------------------
  // CREATE COUNTER IX
  // ---------------------------------------

  const counterIx = new TransactionInstruction({
    programId: COUNTER_PROGRAM_ID,
    keys: [
      {
        pubkey: wallet.publicKey,
        isSigner: true,
        isWritable: true,
      },
      {
        pubkey: counterPda,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ],
    data: Buffer.alloc(0),
  });

  // ---------------------------------------
  // COMPILE
  // ---------------------------------------

  console.log("compiling transaction...");

  const { transaction, remainingAccounts } =
    compileTransaction([counterIx], []);

  // ---------------------------------------
  // QUEUE TASK
  // ---------------------------------------

  console.log("queueing task...");

  const { pubkeys: { task }, signature } =
    await (
      await queueTask(program, {
        taskQueue,
        args: {
          trigger: {
            now: {},
          },

          crankReward: null,

          freeTasks: 0,

          transaction: {
            compiledV0: [transaction],
          },

          description: "increment counter",
        },
      })
    )
      .remainingAccounts(remainingAccounts)
      .rpcAndKeys();

  console.log("task queued!");
  console.log("signature:", signature);
  console.log("task:", task.toBase58());

  // ---------------------------------------
  // MONITOR TASK
  // ---------------------------------------

  console.log("monitoring task...");

  await monitorTask(connection, task);

  console.log("done!");
}

main()
  .then(() => process.exit(0))
  .catch((err) => {
    console.error(err);
    process.exit(1);
  });
