// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@1inch/limit-order-protocol/contracts/interfaces/IOrderMixin.sol";
import "@1inch/limit-order-protocol/contracts/interfaces/IPostInteraction.sol";
import "@1inch/limit-order-protocol/contracts/libraries/OrderLib.sol";
import "./IEscrowFactory.sol";
import "./Escrow.sol";

/**
 * @title Fusion1inchAdapter
 * @dev Integrates 1inch Limit Order Protocol with HTLC for cross-chain swaps
 */
contract Fusion1inchAdapter is IPostInteraction {
    using OrderLib for IOrderMixin.Order;
    
    IOrderMixin public immutable limitOrderProtocol;
    IEscrowFactory public immutable escrowFactory;
    
    // Mapping from order hash to escrow address
    mapping(bytes32 => address) public orderEscrows;
    
    // Events
    event CrossChainOrderCreated(
        bytes32 indexed orderHash,
        address indexed escrow,
        address indexed maker,
        bytes32 secretHash
    );
    
    constructor(address _limitOrderProtocol, address _escrowFactory) {
        limitOrderProtocol = IOrderMixin(_limitOrderProtocol);
        escrowFactory = IEscrowFactory(_escrowFactory);
    }
    
    /**
     * @dev Creates a cross-chain order with HTLC escrow
     * @param order The 1inch limit order
     * @param signature Order signature
     * @param secretHash HTLC secret hash for cross-chain swap
     * @param timeout HTLC timeout
     */
    function createCrossChainOrder(
        IOrderMixin.Order calldata order,
        bytes calldata signature,
        bytes32 secretHash,
        uint256 timeout
    ) external payable returns (bytes32 orderHash, address escrow) {
        // Calculate order hash
        orderHash = order.hash(limitOrderProtocol.DOMAIN_SEPARATOR());
        
        // Verify order signature
        require(
            OrderLib.validateOrder(order, signature, orderHash),
            "Invalid order signature"
        );
        
        // Extract HTLC parameters from order.interaction
        // Order.interaction should contain encoded cross-chain data
        (address crossChainRecipient, uint256 crossChainAmount) = decodeCrossChainData(order.interaction);
        
        // Create HTLC escrow for the order
        escrow = escrowFactory.createEscrow{value: msg.value}(
            order.makerAsset, // token
            order.makingAmount, // amount
            secretHash,
            timeout,
            crossChainRecipient
        );
        
        // Link order to escrow
        orderEscrows[orderHash] = escrow;
        
        emit CrossChainOrderCreated(orderHash, escrow, order.maker, secretHash);
        
        return (orderHash, escrow);
    }
    
    /**
     * @dev Called by 1inch protocol after order fill
     * Handles cross-chain execution
     */
    function fillOrderPostInteraction(
        bytes32 orderHash,
        address maker,
        address taker,
        uint256 makingAmount,
        uint256 takingAmount,
        uint256 remainingMakingAmount,
        bytes memory interaction
    ) external override {
        require(msg.sender == address(limitOrderProtocol), "Only limit order protocol");
        
        // Get linked escrow
        address escrow = orderEscrows[orderHash];
        require(escrow != address(0), "No escrow for order");
        
        // If order is filled, trigger cross-chain notification
        if (remainingMakingAmount == 0) {
            // Emit event for cross-chain relayers
            emit OrderFilledCrossChain(orderHash, escrow, taker);
        }
    }
    
    /**
     * @dev Claim escrow with revealed secret
     */
    function claimWithSecret(bytes32 orderHash, string memory secret) external {
        address escrowAddress = orderEscrows[orderHash];
        require(escrowAddress != address(0), "No escrow for order");
        
        Escrow(escrowAddress).claim(secret);
    }
    
    /**
     * @dev Decode cross-chain data from order interaction field
     */
    function decodeCrossChainData(bytes memory interaction) 
        internal 
        pure 
        returns (address recipient, uint256 amount) 
    {
        // Interaction format: abi.encode(crossChainRecipient, crossChainAmount, chainId)
        (recipient, amount,) = abi.decode(interaction, (address, uint256, uint256));
    }
    
    // Events
    event OrderFilledCrossChain(
        bytes32 indexed orderHash,
        address indexed escrow,
        address taker
    );
}