// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import "forge-std/Script.sol";
import "@1inch/limit-order-protocol/contracts/LimitOrderProtocol.sol";
import "@1inch/solidity-utils/contracts/interfaces/IWETH.sol";

contract DeployLimitOrderProtocol is Script {
    function run() external {
        // Base Sepolia WETH address
        // https://docs.base.org/docs/base-contracts#base-sepolia-testnet
        address weth = 0x4200000000000000000000000000000000000006;

        // For dry run, we can skip private key
        if (vm.envOr("PRIVATE_KEY", uint256(0)) != 0) {
            uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
            vm.startBroadcast(deployerPrivateKey);
        } else {
            // Dry run mode
            vm.startBroadcast();
        }

        LimitOrderProtocol protocol = new LimitOrderProtocol(IWETH(weth));

        vm.stopBroadcast();

        console.log("LimitOrderProtocol deployed to:", address(protocol));
        console.log("Owner:", protocol.owner());
        console.log("WETH address:", weth);
    }
}
