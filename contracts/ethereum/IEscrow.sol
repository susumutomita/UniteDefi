// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title IEscrow
 * @dev Interface for the escrow contract
 */
interface IEscrow {
    function claim(bytes32 secret) external;
    function refund() external;
    function getDetails() external view returns (
        address sender,
        address recipient,
        uint256 amount,
        bytes32 secretHash,
        uint256 deadline,
        uint8 state
    );
}