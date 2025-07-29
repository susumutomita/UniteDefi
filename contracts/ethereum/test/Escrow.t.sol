// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {Escrow} from "../src/Escrow.sol";

contract EscrowTest is Test {
    Escrow public escrow;
    address public sender;
    address public recipient;
    bytes32 public secret;
    bytes32 public secretHash;
    uint256 public amount;
    uint256 public deadline;

    function setUp() public {
        sender = address(0x1);
        recipient = address(0x2);
        secret = "mysecret";
        secretHash = sha256(abi.encodePacked(secret));
        amount = 1 ether;
        deadline = block.timestamp + 3600; // 1 hour from now
        
        // Give sender some ETH
        vm.deal(sender, 10 ether);
        
        // Create escrow contract
        vm.prank(sender);
        escrow = new Escrow{value: amount}(
            sender,
            recipient,
            address(0), // ETH
            amount,
            secretHash,
            deadline
        );
    }

    function testInitialState() public {
        assertEq(escrow.sender(), sender);
        assertEq(escrow.recipient(), recipient);
        assertEq(escrow.amount(), amount);
        assertEq(escrow.secretHash(), secretHash);
        assertEq(escrow.deadline(), deadline);
        assertEq(uint8(escrow.state()), 0); // PENDING
    }

    function testClaimWithCorrectSecret() public {
        uint256 recipientBalanceBefore = recipient.balance;
        
        vm.prank(recipient);
        escrow.claim(secret);
        
        assertEq(uint8(escrow.state()), 1); // CLAIMED
        assertEq(escrow.secret(), secret);
        assertEq(recipient.balance, recipientBalanceBefore + amount);
    }

    function testClaimWithIncorrectSecret() public {
        vm.prank(recipient);
        vm.expectRevert("Invalid secret");
        escrow.claim("wrongsecret");
    }

    function testClaimAfterExpiry() public {
        // Fast forward past deadline
        vm.warp(deadline + 1);
        
        vm.prank(recipient);
        vm.expectRevert("Escrow expired");
        escrow.claim(secret);
    }

    function testRefundBeforeExpiry() public {
        vm.prank(sender);
        vm.expectRevert("Escrow not expired");
        escrow.refund();
    }

    function testRefundAfterExpiry() public {
        uint256 senderBalanceBefore = sender.balance;
        
        // Fast forward past deadline
        vm.warp(deadline + 1);
        
        vm.prank(sender);
        escrow.refund();
        
        assertEq(uint8(escrow.state()), 2); // REFUNDED
        assertEq(sender.balance, senderBalanceBefore + amount);
    }

    function testCannotClaimAfterRefund() public {
        // Fast forward and refund
        vm.warp(deadline + 1);
        vm.prank(sender);
        escrow.refund();
        
        // Try to claim
        vm.prank(recipient);
        vm.expectRevert("Escrow not pending");
        escrow.claim(secret);
    }

    function testCannotRefundAfterClaim() public {
        // Claim first
        vm.prank(recipient);
        escrow.claim(secret);
        
        // Try to refund
        vm.warp(deadline + 1);
        vm.prank(sender);
        vm.expectRevert("Escrow not pending");
        escrow.refund();
    }

    function testGetDetails() public {
        (
            address _sender,
            address _recipient,
            uint256 _amount,
            bytes32 _secretHash,
            uint256 _deadline,
            uint8 _state
        ) = escrow.getDetails();
        
        assertEq(_sender, sender);
        assertEq(_recipient, recipient);
        assertEq(_amount, amount);
        assertEq(_secretHash, secretHash);
        assertEq(_deadline, deadline);
        assertEq(_state, 0); // PENDING
    }
}