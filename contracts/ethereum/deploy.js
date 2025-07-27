const { ethers } = require("hardhat");

async function main() {
  console.log("Deploying EscrowFactory to Sepolia...");

  // Get the contract factory
  const EscrowFactory = await ethers.getContractFactory("EscrowFactory");
  
  // Deploy the contract
  const escrowFactory = await EscrowFactory.deploy();
  await escrowFactory.deployed();

  console.log(`EscrowFactory deployed to: ${escrowFactory.address}`);
  console.log(`Transaction hash: ${escrowFactory.deployTransaction.hash}`);
  
  // Wait for a few block confirmations
  await escrowFactory.deployTransaction.wait(5);
  
  console.log("Deployment confirmed!");
  
  // Verify on Etherscan (optional)
  if (process.env.ETHERSCAN_API_KEY) {
    console.log("Verifying contract on Etherscan...");
    try {
      await hre.run("verify:verify", {
        address: escrowFactory.address,
        constructorArguments: [],
      });
      console.log("Contract verified!");
    } catch (error) {
      console.error("Verification failed:", error);
    }
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });