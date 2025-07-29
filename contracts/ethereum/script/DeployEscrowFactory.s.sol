// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {EscrowFactory} from "../src/EscrowFactory.sol";

contract DeployEscrowFactory is Script {
    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        // Deploy EscrowFactory
        EscrowFactory escrowFactory = new EscrowFactory();

        console.log("EscrowFactory deployed to:", address(escrowFactory));

        vm.stopBroadcast();

        // Log deployment info
        console.log("=== Deployment Summary ===");
        console.log("Network:", block.chainid);
        console.log("Deployer:", msg.sender);
        console.log("EscrowFactory Address:", address(escrowFactory));
        console.log("Block Number:", block.number);
        console.log("Gas Price:", tx.gasprice);
    }
}
