// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "../lib/limit-order-protocol/contracts/interfaces/IOrderMixin.sol";
import "../lib/limit-order-protocol/contracts/interfaces/IPostInteraction.sol";
import "../lib/limit-order-protocol/contracts/libraries/MakerTraitsLib.sol";
import "../lib/limit-order-protocol/contracts/libraries/TakerTraitsLib.sol";
import "../lib/solidity-utils/contracts/libraries/AddressLib.sol";

// Import 1inch official cross-chain swap contracts
import "@1inch/cross-chain-swap/interfaces/IEscrowFactory.sol";
import "@1inch/cross-chain-swap/interfaces/IBaseEscrow.sol";
import "@1inch/cross-chain-swap/libraries/TimelocksLib.sol";

/**
 * @title Fusion1inchNearAdapter
 * @dev Official 1inch Cross-Chain Atomic Swap integration for NEAR Protocol
 * @notice ETHGlobal Unite Hackathon: Extend Fusion+ to Near Prize Implementation
 * @custom:security-contact hackathon@unite-defi.com
 */
contract Fusion1inchNearAdapter is IPostInteraction {
    using MakerTraitsLib for MakerTraits;
    using TakerTraitsLib for TakerTraits;
    using AddressLib for Address;
    using TimelocksLib for Timelocks;

    /// @notice Official 1inch Limit Order Protocol
    IOrderMixin public immutable limitOrderProtocol;

    /// @notice Official 1inch Cross-Chain Escrow Factory
    IEscrowFactory public immutable escrowFactory;

    /// @notice Chain ID for NEAR Protocol (non-EVM)
    uint256 public constant NEAR_CHAIN_ID = 0x4e454152; // "NEAR" in hex

    /// @notice NEAR Protocol integration data
    struct NearSwapData {
        string nearRecipient; // NEAR account ID
        string nearTokenContract; // NEAR token contract
        uint256 nearAmount; // Amount on NEAR side
        bytes32 secretHash; // HTLC secret hash
        bool isActive; // Swap status
        uint256 nearTimestamp; // NEAR timestamp for coordination
    }

    /// @notice Mapping from 1inch order hash to NEAR swap data
    mapping(bytes32 => NearSwapData) public nearSwapData;

    /// @notice Events for NEAR Protocol coordination
    event NearSwapInitiated(
        bytes32 indexed orderHash,
        address indexed maker,
        string nearRecipient,
        string nearTokenContract,
        uint256 nearAmount,
        bytes32 secretHash
    );

    event EthereumToNearSwapFilled(
        bytes32 indexed orderHash,
        address indexed taker,
        uint256 makingAmount,
        uint256 takingAmount,
        string nearRecipient
    );

    event NearToEthereumSecretRevealed(
        bytes32 indexed orderHash,
        bytes32 secret,
        address beneficiary
    );

    constructor(address _limitOrderProtocol, address _escrowFactory) {
        limitOrderProtocol = IOrderMixin(_limitOrderProtocol);
        escrowFactory = IEscrowFactory(_escrowFactory);
    }

    /**
     * @notice Creates a cross-chain swap from Ethereum to NEAR
     * @dev Uses official 1inch Cross-Chain Atomic Swap protocol
     * @param order Official 1inch Order struct with proper MakerTraits
     * @param nearRecipient NEAR account to receive tokens
     * @param nearTokenContract NEAR token contract address
     * @param nearAmount Amount to be locked on NEAR
     * @param secretHash HTLC secret hash for atomic coordination
     * @param timelocks Timelock configuration for escrow
     */
    function initiateEthereumToNearSwap(
        IOrderMixin.Order calldata order,
        string calldata nearRecipient,
        string calldata nearTokenContract,
        uint256 nearAmount,
        bytes32 secretHash,
        Timelocks timelocks
    ) external payable returns (bytes32 orderHash, address escrowSrc) {
        // Validate order has required post-interaction flag
        require(
            order.makerTraits.needPostInteractionCall(),
            "Order must have post-interaction flag for cross-chain"
        );

        // Calculate order hash using official 1inch method
        orderHash = limitOrderProtocol.hashOrder(order);

        // Store NEAR swap data
        nearSwapData[orderHash] = NearSwapData({
            nearRecipient: nearRecipient,
            nearTokenContract: nearTokenContract,
            nearAmount: nearAmount,
            secretHash: secretHash,
            isActive: true,
            nearTimestamp: block.timestamp
        });

        // Pre-compute escrow address for safety deposit
        IBaseEscrow.Immutables memory immutables = IBaseEscrow.Immutables({
            orderHash: orderHash,
            hashlock: secretHash,
            maker: order.maker,
            taker: Address.wrap(uint160(msg.sender)),
            token: order.makerAsset,
            amount: order.makingAmount,
            safetyDeposit: msg.value,
            timelocks: timelocks
        });

        escrowSrc = escrowFactory.addressOfEscrowSrc(immutables);

        emit NearSwapInitiated(
            orderHash,
            order.maker.get(),
            nearRecipient,
            nearTokenContract,
            nearAmount,
            secretHash
        );

        return (orderHash, escrowSrc);
    }

    /**
     * @notice Post-interaction callback from 1inch Limit Order Protocol
     * @dev Called automatically after order fill to trigger NEAR coordination
     */
    function postInteraction(
        IOrderMixin.Order calldata /* order */,
        bytes calldata /* extension */,
        bytes32 orderHash,
        address taker,
        uint256 makingAmount,
        uint256 takingAmount,
        uint256 remainingMakingAmount,
        bytes calldata /* extraData */
    ) external override {
        require(
            msg.sender == address(limitOrderProtocol),
            "Only limit order protocol"
        );

        NearSwapData storage swapData = nearSwapData[orderHash];
        require(swapData.isActive, "No active NEAR swap");

        // Emit event for NEAR relayers to detect and create corresponding HTLC
        emit EthereumToNearSwapFilled(
            orderHash,
            taker,
            makingAmount,
            takingAmount,
            swapData.nearRecipient
        );

        // If completely filled, trigger destination chain escrow creation
        if (remainingMakingAmount == 0) {
            // Additional logic for complete fill
            _triggerNearEscrowCreation(orderHash, swapData);
        }
    }

    /**
     * @notice Creates destination escrow on NEAR protocol
     * @dev This would typically be called by a relayer
     */
    function _triggerNearEscrowCreation(
        bytes32 orderHash,
        NearSwapData storage swapData
    ) internal {
        // Emit detailed event for NEAR relayers
        // Real implementation would use cross-chain messaging
        emit NearSwapInitiated(
            orderHash,
            address(0), // Will be filled by relayer
            swapData.nearRecipient,
            swapData.nearTokenContract,
            swapData.nearAmount,
            swapData.secretHash
        );
    }

    /**
     * @notice Creates official 1inch destination escrow
     * @dev Uses official EscrowFactory.createDstEscrow
     */
    function createDestinationEscrow(
        bytes32 orderHash,
        address maker,
        address token,
        uint256 amount,
        address recipient,
        Timelocks timelocks,
        uint256 srcCancellationTimestamp
    ) external payable {
        NearSwapData storage swapData = nearSwapData[orderHash];
        require(swapData.isActive, "No active swap");

        // Create destination escrow immutables struct
        IBaseEscrow.Immutables memory dstImmutables = IBaseEscrow.Immutables({
            orderHash: orderHash,
            hashlock: swapData.secretHash,
            maker: Address.wrap(uint160(maker)),
            taker: Address.wrap(uint160(recipient)),
            token: Address.wrap(uint160(token)),
            amount: amount,
            safetyDeposit: msg.value,
            timelocks: timelocks
        });

        // Create destination escrow using official 1inch factory
        escrowFactory.createDstEscrow(dstImmutables, srcCancellationTimestamp);
    }

    /**
     * @notice Handles secret revelation from NEAR Protocol
     * @param orderHash Hash of the 1inch order
     * @param secret Revealed secret from NEAR HTLC
     */
    function revealSecretFromNear(bytes32 orderHash, bytes32 secret) external {
        NearSwapData storage swapData = nearSwapData[orderHash];
        require(swapData.isActive, "Swap not active");

        // Verify secret matches hash
        require(
            keccak256(abi.encodePacked(secret)) == swapData.secretHash,
            "Invalid secret"
        );

        emit NearToEthereumSecretRevealed(orderHash, secret, msg.sender);

        // Mark swap as completed
        swapData.isActive = false;
    }

    /**
     * @notice Helper to create MakerTraits for cross-chain orders
     * @dev Ensures proper flags for 1inch cross-chain integration
     */
    function createNearCrossChainMakerTraits(
        uint256 expiration,
        address allowedSender,
        bool allowPartialFills
    ) external pure returns (MakerTraits) {
        uint256 traits = 0;

        // Set post-interaction flag (required for cross-chain)
        traits |= (1 << 251); // POST_INTERACTION_CALL_FLAG

        // Set extension flag for cross-chain data
        traits |= (1 << 249); // HAS_EXTENSION_FLAG

        // Set partial fills flag if needed
        if (!allowPartialFills) {
            traits |= (1 << 255); // NO_PARTIAL_FILLS_FLAG
        } else {
            traits |= (1 << 254); // ALLOW_MULTIPLE_FILLS_FLAG
        }

        // Set expiration if provided
        if (expiration > 0) {
            traits |= (expiration & type(uint40).max) << 80;
        }

        // Set allowed sender if provided
        if (allowedSender != address(0)) {
            traits |= uint256(uint160(allowedSender)) & type(uint80).max;
        }

        return MakerTraits.wrap(traits);
    }

    /**
     * @notice Submits order directly to 1inch Limit Order Protocol
     * @dev Convenience function for testing and integration
     */
    function submitOrderTo1inch(
        IOrderMixin.Order calldata order,
        bytes32 r,
        bytes32 vs,
        uint256 amount,
        TakerTraits takerTraits
    )
        external
        payable
        returns (uint256 makingAmount, uint256 takingAmount, bytes32 orderHash)
    {
        return
            limitOrderProtocol.fillOrder{value: msg.value}(
                order,
                r,
                vs,
                amount,
                takerTraits
            );
    }

    /**
     * @notice Gets NEAR swap data for a given order
     */
    function getNearSwapData(
        bytes32 orderHash
    ) external view returns (NearSwapData memory) {
        return nearSwapData[orderHash];
    }

    /**
     * @notice Validates order for NEAR cross-chain compatibility
     */
    function isValidNearOrder(
        IOrderMixin.Order calldata order
    ) external pure returns (bool) {
        return
            order.makerTraits.needPostInteractionCall() &&
            order.makerTraits.hasExtension();
    }

    /**
     * @notice Emergency function to cancel inactive swaps
     */
    function cancelNearSwap(bytes32 orderHash) external {
        NearSwapData storage swapData = nearSwapData[orderHash];
        require(swapData.isActive, "Swap not active");

        // Allow cancellation after timeout
        require(
            block.timestamp > swapData.nearTimestamp + 3600, // 1 hour timeout
            "Timeout not reached"
        );

        swapData.isActive = false;
    }
}
