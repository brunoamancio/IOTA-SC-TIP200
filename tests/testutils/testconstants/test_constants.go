package testconstants

const (
	// ContractName is defined in smartcontract/rust/Cargo.toml > package > name
	ContractName = "erc_721"
	// Debug is used by Solo. 'true' for logging level 'debug', otherwise 'info'
	Debug = false
	// StackTrace is used by Solo. 'true' if stack trace must be printed in case of errors
	StackTrace = false

	// AccountsContractName sets the name of the Accounts contract, which is a root contract present in every chain
	AccountsContractName = "accounts"

	//InitialWalletFunds is use to fund address in NewSignatureSchemeWithFunds. // Defined in iotaledger/wasp/packages/testutiltestutil.RequestFundsAmount.
	InitialWalletFunds = 1337

	// TransferValueIotas is the default amount of IOTAs to transfer in unit tests.
	TransferValueIotas = 1000

	// IotaTokensConsumedByRequest -> INTERESTING FACT: Calls to a smart contract require 1 EXTRA iota token to be sent to the chain it is located in.
	/* It is colored with the chain´s color and corresponds to the request. That is how the protocol locates the backlog of
	requests to be processed. Basically, it works as a flag. After the request is processed, the token is uncolored
	and sent to the chain owner´s account in the chain.
	*/
	IotaTokensConsumedByRequest = 1

	// IotaTokensConsumedByChain -> INTERESTING FACT: Creating a chain requires 2 iota tokens. They are colored with the chain's color,
	// 1 is sent to the chain's address in the value tangle, and the other is used exactly as iotaTokensConsumedByRequest.
	IotaTokensConsumedByChain = 1
)
