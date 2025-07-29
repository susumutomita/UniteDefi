// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {EscrowFactory} from "../src/EscrowFactory.sol";
import {Escrow} from "../src/Escrow.sol";

contract EscrowFactoryTest is Test {
    EscrowFactory public factory;
    address public sender;
    address public recipient;
    bytes32 public secretHash;
    uint256 public amount;
    uint256 public timeout;

    function setUp() public {
        factory = new EscrowFactory();
        sender = address(0x1);
        recipient = address(0x2);
        secretHash = keccak256("secret");
        amount = 1 ether;
        timeout = 3600; // 1 hour
        
        // Give sender some ETH
        vm.deal(sender, 10 ether);
    }

    function testCreateEscrowETH() public {
        vm.prank(sender);
        address escrowAddress = factory.createEscrow{value: amount}(
            address(0), // ETH
            amount,
            secretHash,
            timeout,
            recipient
        );
        
        assertTrue(escrowAddress != address(0));
        
        // Check escrow state
        Escrow escrow = Escrow(payable(escrowAddress));
        assertEq(escrow.sender(), sender);
        assertEq(escrow.recipient(), recipient);
        assertEq(escrow.amount(), amount);
        assertEq(escrow.secretHash(), secretHash);
    }

    function testCreateEscrowInvalidRecipient() public {
        vm.prank(sender);
        vm.expectRevert("Invalid recipient");
        factory.createEscrow{value: amount}(
            address(0),
            amount,
            secretHash,
            timeout,
            address(0) // Invalid recipient
        );
    }

    function testCreateEscrowInvalidTimeout() public {
        vm.prank(sender);
        vm.expectRevert("Invalid timeout");
        factory.createEscrow{value: amount}(
            address(0),
            amount,
            secretHash,
            0, // Invalid timeout
            recipient
        );
    }

    function testCreateEscrowIncorrectETHAmount() public {
        vm.prank(sender);
        vm.expectRevert("Incorrect ETH amount");
        factory.createEscrow{value: amount - 1}( // Send less ETH
            address(0),
            amount,
            secretHash,
            timeout,
            recipient
        );
    }
}