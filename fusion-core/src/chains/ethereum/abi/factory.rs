pub use i_escrow_factory::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod i_escrow_factory {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ESCROW_DST_IMPLEMENTATION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ESCROW_DST_IMPLEMENTATION",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ESCROW_SRC_IMPLEMENTATION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ESCROW_SRC_IMPLEMENTATION",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addressOfEscrowDst"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addressOfEscrowDst"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("immutables"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IBaseEscrow.Immutables",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addressOfEscrowSrc"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addressOfEscrowSrc"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("immutables"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IBaseEscrow.Immutables",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createDstEscrow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createDstEscrow"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dstImmutables"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IBaseEscrow.Immutables",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "srcCancellationTimestamp",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DstEscrowCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DstEscrowCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("escrow"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("hashlock"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("taker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SrcEscrowCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SrcEscrowCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("srcImmutables"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "dstImmutablesComplement",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientEscrowBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientEscrowBalance",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidCreationTime"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidCreationTime",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidPartialFill"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidPartialFill"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSecretsAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidSecretsAmount",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IESCROWFACTORY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IEscrowFactory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IEscrowFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IEscrowFactory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IEscrowFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IEscrowFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IEscrowFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IEscrowFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IESCROWFACTORY_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `ESCROW_DST_IMPLEMENTATION` (0xba551177) function
        pub fn escrow_dst_implementation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([186, 85, 17, 119], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ESCROW_SRC_IMPLEMENTATION` (0x7040f173) function
        pub fn escrow_src_implementation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([112, 64, 241, 115], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addressOfEscrowDst` (0xbe58e91c) function
        pub fn address_of_escrow_dst(
            &self,
            immutables: Immutables,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([190, 88, 233, 28], (immutables,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addressOfEscrowSrc` (0xfb6bd47e) function
        pub fn address_of_escrow_src(
            &self,
            immutables: Immutables,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([251, 107, 212, 126], (immutables,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createDstEscrow` (0xdea024e4) function
        pub fn create_dst_escrow(
            &self,
            dst_immutables: Immutables,
            src_cancellation_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [222, 160, 36, 228],
                    (dst_immutables, src_cancellation_timestamp),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DstEscrowCreated` event
        pub fn dst_escrow_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DstEscrowCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SrcEscrowCreated` event
        pub fn src_escrow_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SrcEscrowCreatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IEscrowFactoryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IEscrowFactory<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InsufficientEscrowBalance` with signature `InsufficientEscrowBalance()` and selector `0x34f5151d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InsufficientEscrowBalance", abi = "InsufficientEscrowBalance()")]
    pub struct InsufficientEscrowBalance;
    ///Custom Error type `InvalidCreationTime` with signature `InvalidCreationTime()` and selector `0xf4840e96`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidCreationTime", abi = "InvalidCreationTime()")]
    pub struct InvalidCreationTime;
    ///Custom Error type `InvalidPartialFill` with signature `InvalidPartialFill()` and selector `0xeab3a174`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidPartialFill", abi = "InvalidPartialFill()")]
    pub struct InvalidPartialFill;
    ///Custom Error type `InvalidSecretsAmount` with signature `InvalidSecretsAmount()` and selector `0x10d629d3`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidSecretsAmount", abi = "InvalidSecretsAmount()")]
    pub struct InvalidSecretsAmount;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IEscrowFactoryErrors {
        InsufficientEscrowBalance(InsufficientEscrowBalance),
        InvalidCreationTime(InvalidCreationTime),
        InvalidPartialFill(InvalidPartialFill),
        InvalidSecretsAmount(InvalidSecretsAmount),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for IEscrowFactoryErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <InsufficientEscrowBalance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientEscrowBalance(decoded));
            }
            if let Ok(decoded) = <InvalidCreationTime as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidCreationTime(decoded));
            }
            if let Ok(decoded) = <InvalidPartialFill as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidPartialFill(decoded));
            }
            if let Ok(decoded) = <InvalidSecretsAmount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidSecretsAmount(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IEscrowFactoryErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InsufficientEscrowBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidCreationTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidPartialFill(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSecretsAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for IEscrowFactoryErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InsufficientEscrowBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidCreationTime as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidPartialFill as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidSecretsAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for IEscrowFactoryErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InsufficientEscrowBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidCreationTime(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidPartialFill(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidSecretsAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for IEscrowFactoryErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InsufficientEscrowBalance> for IEscrowFactoryErrors {
        fn from(value: InsufficientEscrowBalance) -> Self {
            Self::InsufficientEscrowBalance(value)
        }
    }
    impl ::core::convert::From<InvalidCreationTime> for IEscrowFactoryErrors {
        fn from(value: InvalidCreationTime) -> Self {
            Self::InvalidCreationTime(value)
        }
    }
    impl ::core::convert::From<InvalidPartialFill> for IEscrowFactoryErrors {
        fn from(value: InvalidPartialFill) -> Self {
            Self::InvalidPartialFill(value)
        }
    }
    impl ::core::convert::From<InvalidSecretsAmount> for IEscrowFactoryErrors {
        fn from(value: InvalidSecretsAmount) -> Self {
            Self::InvalidSecretsAmount(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "DstEscrowCreated",
        abi = "DstEscrowCreated(address,bytes32,uint256)"
    )]
    pub struct DstEscrowCreatedFilter {
        pub escrow: ::ethers::core::types::Address,
        pub hashlock: [u8; 32],
        pub taker: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "SrcEscrowCreated",
        abi = "SrcEscrowCreated((bytes32,bytes32,uint256,uint256,uint256,uint256,uint256,uint256),(uint256,uint256,uint256,uint256,uint256))"
    )]
    pub struct SrcEscrowCreatedFilter {
        pub src_immutables: Immutables,
        pub dst_immutables_complement: DstImmutablesComplement,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IEscrowFactoryEvents {
        DstEscrowCreatedFilter(DstEscrowCreatedFilter),
        SrcEscrowCreatedFilter(SrcEscrowCreatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for IEscrowFactoryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DstEscrowCreatedFilter::decode_log(log) {
                return Ok(IEscrowFactoryEvents::DstEscrowCreatedFilter(decoded));
            }
            if let Ok(decoded) = SrcEscrowCreatedFilter::decode_log(log) {
                return Ok(IEscrowFactoryEvents::SrcEscrowCreatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IEscrowFactoryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DstEscrowCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SrcEscrowCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DstEscrowCreatedFilter> for IEscrowFactoryEvents {
        fn from(value: DstEscrowCreatedFilter) -> Self {
            Self::DstEscrowCreatedFilter(value)
        }
    }
    impl ::core::convert::From<SrcEscrowCreatedFilter> for IEscrowFactoryEvents {
        fn from(value: SrcEscrowCreatedFilter) -> Self {
            Self::SrcEscrowCreatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `ESCROW_DST_IMPLEMENTATION` function with signature `ESCROW_DST_IMPLEMENTATION()` and selector `0xba551177`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ESCROW_DST_IMPLEMENTATION", abi = "ESCROW_DST_IMPLEMENTATION()")]
    pub struct EscrowDstImplementationCall;
    ///Container type for all input parameters for the `ESCROW_SRC_IMPLEMENTATION` function with signature `ESCROW_SRC_IMPLEMENTATION()` and selector `0x7040f173`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ESCROW_SRC_IMPLEMENTATION", abi = "ESCROW_SRC_IMPLEMENTATION()")]
    pub struct EscrowSrcImplementationCall;
    ///Container type for all input parameters for the `addressOfEscrowDst` function with signature `addressOfEscrowDst((bytes32,bytes32,uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0xbe58e91c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "addressOfEscrowDst",
        abi = "addressOfEscrowDst((bytes32,bytes32,uint256,uint256,uint256,uint256,uint256,uint256))"
    )]
    pub struct AddressOfEscrowDstCall {
        pub immutables: Immutables,
    }
    ///Container type for all input parameters for the `addressOfEscrowSrc` function with signature `addressOfEscrowSrc((bytes32,bytes32,uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0xfb6bd47e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "addressOfEscrowSrc",
        abi = "addressOfEscrowSrc((bytes32,bytes32,uint256,uint256,uint256,uint256,uint256,uint256))"
    )]
    pub struct AddressOfEscrowSrcCall {
        pub immutables: Immutables,
    }
    ///Container type for all input parameters for the `createDstEscrow` function with signature `createDstEscrow((bytes32,bytes32,uint256,uint256,uint256,uint256,uint256,uint256),uint256)` and selector `0xdea024e4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "createDstEscrow",
        abi = "createDstEscrow((bytes32,bytes32,uint256,uint256,uint256,uint256,uint256,uint256),uint256)"
    )]
    pub struct CreateDstEscrowCall {
        pub dst_immutables: Immutables,
        pub src_cancellation_timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IEscrowFactoryCalls {
        EscrowDstImplementation(EscrowDstImplementationCall),
        EscrowSrcImplementation(EscrowSrcImplementationCall),
        AddressOfEscrowDst(AddressOfEscrowDstCall),
        AddressOfEscrowSrc(AddressOfEscrowSrcCall),
        CreateDstEscrow(CreateDstEscrowCall),
    }
    impl ::ethers::core::abi::AbiDecode for IEscrowFactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <EscrowDstImplementationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EscrowDstImplementation(decoded));
            }
            if let Ok(decoded) = <EscrowSrcImplementationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EscrowSrcImplementation(decoded));
            }
            if let Ok(decoded) = <AddressOfEscrowDstCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressOfEscrowDst(decoded));
            }
            if let Ok(decoded) = <AddressOfEscrowSrcCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressOfEscrowSrc(decoded));
            }
            if let Ok(decoded) = <CreateDstEscrowCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateDstEscrow(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IEscrowFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::EscrowDstImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EscrowSrcImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddressOfEscrowDst(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddressOfEscrowSrc(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateDstEscrow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IEscrowFactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EscrowDstImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EscrowSrcImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddressOfEscrowDst(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddressOfEscrowSrc(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateDstEscrow(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EscrowDstImplementationCall> for IEscrowFactoryCalls {
        fn from(value: EscrowDstImplementationCall) -> Self {
            Self::EscrowDstImplementation(value)
        }
    }
    impl ::core::convert::From<EscrowSrcImplementationCall> for IEscrowFactoryCalls {
        fn from(value: EscrowSrcImplementationCall) -> Self {
            Self::EscrowSrcImplementation(value)
        }
    }
    impl ::core::convert::From<AddressOfEscrowDstCall> for IEscrowFactoryCalls {
        fn from(value: AddressOfEscrowDstCall) -> Self {
            Self::AddressOfEscrowDst(value)
        }
    }
    impl ::core::convert::From<AddressOfEscrowSrcCall> for IEscrowFactoryCalls {
        fn from(value: AddressOfEscrowSrcCall) -> Self {
            Self::AddressOfEscrowSrc(value)
        }
    }
    impl ::core::convert::From<CreateDstEscrowCall> for IEscrowFactoryCalls {
        fn from(value: CreateDstEscrowCall) -> Self {
            Self::CreateDstEscrow(value)
        }
    }
    ///Container type for all return fields from the `ESCROW_DST_IMPLEMENTATION` function with signature `ESCROW_DST_IMPLEMENTATION()` and selector `0xba551177`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EscrowDstImplementationReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `ESCROW_SRC_IMPLEMENTATION` function with signature `ESCROW_SRC_IMPLEMENTATION()` and selector `0x7040f173`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EscrowSrcImplementationReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `addressOfEscrowDst` function with signature `addressOfEscrowDst((bytes32,bytes32,uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0xbe58e91c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AddressOfEscrowDstReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `addressOfEscrowSrc` function with signature `addressOfEscrowSrc((bytes32,bytes32,uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0xfb6bd47e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AddressOfEscrowSrcReturn(pub ::ethers::core::types::Address);
    ///`Immutables(bytes32,bytes32,uint256,uint256,uint256,uint256,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Immutables {
        pub order_hash: [u8; 32],
        pub hashlock: [u8; 32],
        pub maker: ::ethers::core::types::U256,
        pub taker: ::ethers::core::types::U256,
        pub token: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
        pub safety_deposit: ::ethers::core::types::U256,
        pub timelocks: ::ethers::core::types::U256,
    }
    ///`DstImmutablesComplement(uint256,uint256,uint256,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DstImmutablesComplement {
        pub maker: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
        pub token: ::ethers::core::types::U256,
        pub safety_deposit: ::ethers::core::types::U256,
        pub chain_id: ::ethers::core::types::U256,
    }
}
