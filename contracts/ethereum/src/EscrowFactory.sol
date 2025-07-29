// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./IEscrowFactory.sol";
import "./Escrow.sol";

/**
 * @title EscrowFactory
 * @dev Factory contract for creating HTLC escrow contracts compatible with 1inch Fusion+
 */
contract EscrowFactory is IEscrowFactory {
    mapping(bytes32 => address) public escrows;

    event EscrowCreated(
        bytes32 indexed escrowId,
        address indexed escrow,
        address indexed sender,
        address recipient,
        address token,
        uint256 amount,
        bytes32 secretHash,
        uint256 timeout
    );

    /**
     * @dev Creates a new escrow contract
     * @param token The token to be escrowed (address(0) for ETH)
     * @param amount The amount to be escrowed
     * @param secretHash The hash of the secret
     * @param timeout The timeout period in seconds
     * @param recipient The recipient address
     * @return escrow The address of the created escrow contract
     */
    function createEscrow(
        address token,
        uint256 amount,
        bytes32 secretHash,
        uint256 timeout,
        address recipient
    ) external payable returns (address escrow) {
        require(secretHash != bytes32(0), "Invalid secret hash");
        require(timeout > 0, "Invalid timeout");
        require(recipient != address(0), "Invalid recipient");

        if (token == address(0)) {
            require(msg.value == amount, "Incorrect ETH amount");
        }

        // Generate unique escrow ID
        bytes32 escrowId = keccak256(
            abi.encodePacked(
                msg.sender,
                recipient,
                token,
                amount,
                secretHash,
                timeout,
                block.timestamp
            )
        );

        require(escrows[escrowId] == address(0), "Escrow already exists");

        // Deploy new escrow contract
        escrow = address(
            new Escrow{value: msg.value}(
                msg.sender,
                recipient,
                token,
                amount,
                secretHash,
                block.timestamp + timeout
            )
        );

        escrows[escrowId] = escrow;

        emit EscrowCreated(
            escrowId,
            escrow,
            msg.sender,
            recipient,
            token,
            amount,
            secretHash,
            timeout
        );

        return escrow;
    }

    /**
     * @dev Get escrow address by ID
     * @param escrowId The escrow ID
     * @return The escrow contract address
     */
    function getEscrow(bytes32 escrowId) external view returns (address) {
        return escrows[escrowId];
    }
}
