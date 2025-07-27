// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title IEscrowFactory
 * @dev Interface for the escrow factory contract
 */
interface IEscrowFactory {
    function createEscrow(
        address token,
        uint256 amount,
        bytes32 secretHash,
        uint256 timeout,
        address recipient
    ) external payable returns (address escrow);
    
    function getEscrow(bytes32 escrowId) external view returns (address);
}