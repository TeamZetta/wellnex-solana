import * as solanaWeb3 from '@solana/web3.js'

// connect to wallet
let secretKey = Uint8Array.from([186, 51, 127, 107, 49, 133, 39, 198, 188, 6, 56, 99, 148, 61, 55, 105, 183, 174, 22, 148, 94, 130, 151, 26, 97, 242, 86, 11, 228, 166, 83, 151, 168, 168, 35, 27, 252, 211, 180, 7, 189, 232, 51, 111, 88, 185, 215, 102, 24, 68, 160, 68, 175, 3, 199, 212, 43, 237, 184, 123, 143, 237, 44, 31])

let keypair = solanaWeb3.keypair.fromSecretKey(secretKey)
console.log(keypair)