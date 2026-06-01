import { AnchorProvider, Wallet } from "@coral-xyz/anchor";
import {compileTransaction, init, queueTask} from "@helium/tuktuk-sdk";
import { clusterApiUrl, Connection, Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from "@solana/web3.js";

let PROGRAM_ID=new PublicKey("5xLw1uD9fEM4rgayTBt1PBxR9BNL1iLfn5xXMc6XWZUq");
let connection=new Connection(clusterApiUrl("devnet"),"confirmed");

async function addTask(taskQueuePubkey:string){
    let user=Keypair.fromSecretKey(Uint8Array.from([48,182,182,234,169,224,236,113,52,199,47,66,39,2,163,52,183,44,45,27,127,49,133,151,64,70,248,16,46,218,234,198,42,180,5,68,243,235,189,56,197,37,17,85,205,189,100,191,64,74,171,3,37,193,199,195,213,54,156,198,228,15,248,188]));
    console.log("user : ",user.publicKey.toBase58());
    
    let [counterPda,bump]=PublicKey.findProgramAddressSync([Buffer.from("counter")],PROGRAM_ID);
    console.log("counter pda : ",counterPda.toBase58());

    let ix=new TransactionInstruction({
        programId:PROGRAM_ID,
        keys:[
            {pubkey:user.publicKey, isSigner:true,isWritable:true},
            {pubkey:counterPda, isSigner:false,isWritable:true},
            {pubkey:SystemProgram.programId, isSigner:false,isWritable:false},
        ],
        data:Buffer.from([])
    });
    let tx=new Transaction().add(ix);
    tx.recentBlockhash=(await connection.getLatestBlockhash()).blockhash;

    let wallet=new Wallet(user);
    let provider=new AnchorProvider(connection,wallet,{
        commitment:"confirmed"
    });
    let program=await init(provider);

    let task_queue_pubkey=new PublicKey(taskQueuePubkey);

     const { transaction, remainingAccounts } = compileTransaction([ix],[]);

    let {pubkeys,signature}=await (await queueTask(program,{
        taskQueue:task_queue_pubkey,
        args:{
            trigger:{now:{}},
            crankReward: null,
            freeTasks: 0,
            transaction: {
                compiledV0: [transaction],
            },
            description: `tx done`,
        }
    })   
).remainingAccounts(remainingAccounts).rpcAndKeys();

    console.log("task queued!");
    console.log("signature:", signature);
    // console.log("task:", task.toBase58());
}

addTask("Grk2H1LCeb6NTyfKSsg7s7PXDpVTLq6oCfaLTcxJxhBo");