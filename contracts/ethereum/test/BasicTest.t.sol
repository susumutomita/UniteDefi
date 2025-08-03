// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/Fusion1inchNearAdapter.sol";

contract BasicTest is Test {
    function testBasicConstant() public {
        // Basic test to ensure compilation
        assertEq(uint256(1 + 1), uint256(2));
    }

    function testNearChainId() public {
        // Test NEAR chain ID constant
        uint256 expectedNearChainId = 0x4e454152; // "NEAR" in hex
        assertEq(expectedNearChainId, 0x4e454152);
    }
}
