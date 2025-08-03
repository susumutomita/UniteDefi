// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Script.sol";
import "../src/Fusion1inchNearAdapter.sol";

contract DeployFusion1inchAdapter is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        // Use existing deployed contracts on Base Sepolia
        address limitOrderProtocol = 0x171C87724E720F2806fc29a010a62897B30fdb62;
        address escrowFactory = 0x848285f35044e485BD5F0235c27924b1392144b3;

        // Deploy Fusion1inchNearAdapter
        Fusion1inchNearAdapter adapter = new Fusion1inchNearAdapter(limitOrderProtocol, escrowFactory);

        vm.stopBroadcast();

        console.log("Fusion1inchNearAdapter deployed at:", address(adapter));
        console.log("Constructor args:");
        console.log("  limitOrderProtocol:", limitOrderProtocol);
        console.log("  escrowFactory:", escrowFactory);
    }
}
