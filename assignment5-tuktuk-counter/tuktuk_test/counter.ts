import { clusterApiUrl, Connection, PublicKey } from "@solana/web3.js";

let connection=new Connection(clusterApiUrl("devnet"),"confirmed");
async function main(){
    let counter_pda=new PublicKey("7Kvw96E8UKCKikb3QmHQLGYJp7Hg2E5zPMPgwrJMfYZN");
    let data=await connection.getAccountInfo(counter_pda,"confirmed");
    console.log("counter pda data : ",data?.data);
}
main();


//tuktuk -u https://api.devnet.solana.com task-queue close --task-queue-id 301

//tuktuk -u https://api.devnet.solana.com task-queue remove-queue-authority --task-queue-name counter-queue1 --queue-authority $(solana address)

//tuktuk -u https://api.devnet.solana.com task-queue create --name counter-queue1 --capacity 10 --funding-amount 10000000 --queue-authority $(
// solana address) --min-crank-reward 100000 --stale-task-age 1000
// {
//   "pubkey": "3eywazceoS78cp9Fd9Nvcjvqxr6drWgRiKNrEkd9xKFQ",
//   "id": 301,
//   "capacity": 10,
//   "update_authority": "3shLPzr2Dd4d8XShBMrcUnUUoRTf1iEmDDaTXLiBLAC3",
//   "name": "counter-queue1",
//   "min_crank_reward": 100000,
//   "balance": 1010000000,
//   "stale_task_age": 1000
// }

// tuktuk-crank-turner -c config.toml

//bun run index2 --queueName counter-queue1 --walletPath ~/.config/solana/id.json --rpcUrl https://api.devnet.solana.com --programId 5xLw1uD9fEM4rgayTBt1PBxR9BNL1iLfn5xXMc6XWZUq

// tuktuk -u https://api.devnet.solana.com task list --task-queue-name counter-queue3

//  tuktuk -u https://api.devnet.solana.com task run --task-queue-name counter-queue5 --description "increment"
// Tx sent: 3hEXsTe5hQ9gAvd3WS8NkLQtTkefDPC6rCmsxkJVAjo1KEuQm6pWxEDmdsgBE5SG3VL4A4vWegfWzs1SqbkNZwWi