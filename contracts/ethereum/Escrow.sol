// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./IEscrow.sol";

interface IERC20 {
    function transfer(address to, uint256 amount) external returns (bool);
    function transferFrom(
        address from,
        address to,
        uint256 amount
    ) external returns (bool);
}

/**
 * @title Escrow
 * @dev HTLC escrow contract for atomic swaps
 */
contract Escrow is IEscrow {
    enum State {
        PENDING,
        CLAIMED,
        REFUNDED
    }

    address public immutable sender;
    address public immutable recipient;
    address public immutable token;
    uint256 public immutable amount;
    bytes32 public immutable secretHash;
    uint256 public immutable deadline;

    State public state;
    bytes32 public secret;

    event Claimed(address indexed recipient, bytes32 secret);
    event Refunded(address indexed sender);

    modifier onlyPending() {
        require(state == State.PENDING, "Escrow not pending");
        _;
    }

    constructor(
        address _sender,
        address _recipient,
        address _token,
        uint256 _amount,
        bytes32 _secretHash,
        uint256 _deadline
    ) payable {
        sender = _sender;
        recipient = _recipient;
        token = _token;
        amount = _amount;
        secretHash = _secretHash;
        deadline = _deadline;
        state = State.PENDING;

        // If ERC20 token, transfer from sender
        if (_token != address(0)) {
            require(
                IERC20(_token).transferFrom(_sender, address(this), _amount),
                "Token transfer failed"
            );
        }
    }

    /**
     * @dev Claim the escrow by revealing the secret
     * @param _secret The secret that hashes to secretHash
     */
    function claim(bytes32 _secret) external onlyPending {
        require(
            sha256(abi.encodePacked(_secret)) == secretHash,
            "Invalid secret"
        );
        require(block.timestamp <= deadline, "Escrow expired");

        state = State.CLAIMED;
        secret = _secret;

        // Transfer funds to recipient
        if (token == address(0)) {
            // Transfer ETH
            (bool success, ) = recipient.call{value: amount}("");
            require(success, "ETH transfer failed");
        } else {
            // Transfer ERC20
            require(
                IERC20(token).transfer(recipient, amount),
                "Token transfer failed"
            );
        }

        emit Claimed(recipient, _secret);
    }

    /**
     * @dev Refund the escrow after timeout
     */
    function refund() external onlyPending {
        require(block.timestamp > deadline, "Escrow not expired");

        state = State.REFUNDED;

        // Transfer funds back to sender
        if (token == address(0)) {
            // Transfer ETH
            (bool success, ) = sender.call{value: amount}("");
            require(success, "ETH transfer failed");
        } else {
            // Transfer ERC20
            require(
                IERC20(token).transfer(sender, amount),
                "Token transfer failed"
            );
        }

        emit Refunded(sender);
    }

    /**
     * @dev Get escrow details
     */
    function getDetails()
        external
        view
        returns (
            address _sender,
            address _recipient,
            uint256 _amount,
            bytes32 _secretHash,
            uint256 _deadline,
            uint8 _state
        )
    {
        return (sender, recipient, amount, secretHash, deadline, uint8(state));
    }
}
