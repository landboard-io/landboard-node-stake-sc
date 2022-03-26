##### - configuration - #####
PROXY=https://testnet-gateway.elrond.com
CHAIN_ID="T"

WALLET="./wallets/test-wallet.pem"
WALLET2="./wallets/test-wallet-2.pem"

CALLER_ADDRESS="erd1dl8ucerztz80eqtvs2u35vj5pckle3h3mnuce5fctyzxp4d74dfqwy7ntn"
CALLER_ADDRESS_HEX="0x$(erdpy wallet bech32 --decode ${CALLER_ADDRESS})"
CALLER_ADDRESS_ONLY_HEX="$(erdpy wallet bech32 --decode ${CALLER_ADDRESS})"

CALLER2_ADDRESS="erd1ygdttzrulwfspme2s4qrx5y2qyfqalju0k2vcyy8z3979whlj9qssl5uay"
CALLER2_ADDRESS_HEX="0x$(erdpy wallet bech32 --decode ${CALLER2_ADDRESS})"
CALLER2_ADDRESS_ONLY_HEX="$(erdpy wallet bech32 --decode ${CALLER2_ADDRESS})"

TOKEN_ID="SVEN-0deee5"
TOKEN_ID_HEX="0x$(echo -n ${TOKEN_ID} | xxd -p -u | tr -d '\n')"
TOKEN_ID_ONLY_HEX="$(echo -n ${TOKEN_ID} | xxd -p -u | tr -d '\n')"

REFERRRAL_ACTIVATION_AMOUNT=300000000000000000000 # 300 LAND
APY_INCREASE_PER_REFERRAL=50    # 0.5%
MAX_APY_INCREASE_BY_REFERRAL=1000   # 10%
PROMO_INCREASE_APY=200 # 2%
TOTAL_REFERRAL_COUNT_LIMIT=1000

STAKE="stake"
STAKE_ONLY_HEX="$(echo -n ${STAKE} | xxd -p -u | tr -d '\n')"

#####
ADDRESS=$(erdpy data load --key=address-devnet)
TRANSACTION=$(erdpy data load --key=deployTransaction-devnet)
######

deploy() {
    erdpy --verbose contract deploy \
    --project=${PROJECT} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=100000000 \
    --arguments ${TOKEN_ID_HEX} ${TOKEN_ID_HEX} ${REFERRRAL_ACTIVATION_AMOUNT} ${APY_INCREASE_PER_REFERRAL} ${MAX_APY_INCREASE_BY_REFERRAL} ${PROMO_INCREASE_APY} ${TOTAL_REFERRAL_COUNT_LIMIT} \
    --send \
    --metadata-payable \
    --outfile="deploy-devnet.interaction.json" \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} || return

    TRANSACTION=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    erdpy data store --key=address-devnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

addStakeTypes() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="addStakeTypes" \
    --arguments 1 1 100000000000000000000 5000 5000 30 30 100000000000000000000 5000 10000 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

stake() {
    erdpy --verbose tx new --receiver ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=10000000 \
    --data="ESDTTransfer@${TOKEN_ID_ONLY_HEX}@1043561a8829300000@${STAKE_ONLY_HEX}@01@${CALLER2_ADDRESS_ONLY_HEX}" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

stakeWithoutReferral() {
    erdpy --verbose tx new --receiver ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=10000000 \
    --data="ESDTTransfer@${TOKEN_ID_ONLY_HEX}@021e19e0c9bab2400000@${STAKE_ONLY_HEX}@01" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

unstake() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="unstake" \
    --arguments 1 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

claim() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="claim" \
    --arguments 1 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

stake2() {
    erdpy --verbose tx new --receiver ${ADDRESS} \
    --recall-nonce --pem=${WALLET2} \
    --gas-limit=20000000 \
    --data="ESDTTransfer@${TOKEN_ID_ONLY_HEX}@056bc75e2d63100000@${STAKE_ONLY_HEX}@01@${CALLER_ADDRESS_ONLY_HEX}" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

unstake2() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET2} \
    --gas-limit=6000000 \
    --function="unstake" \
    --arguments 1 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

claim2() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET2} \
    --gas-limit=6000000 \
    --function="claim" \
    --arguments 1 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

withdraw() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="withdraw" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

setReferralActivationAmount() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="setReferralActivationAmount" \
    --arguments 1000000000000000000 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

setApyIncreasePerReferral() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="setApyIncreasePerReferral" \
    --arguments 50 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

setMaxApyIncreaseByReferral() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="setMaxApyIncreaseByReferral" \
    --arguments 1000 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

# config

getStakeTypes() {
    erdpy --verbose contract query ${ADDRESS} --function="getStakeTypes" --proxy=${PROXY}
}

getStakerAddresses() {
    erdpy --verbose contract query ${ADDRESS} --function="getStakerAddresses" --proxy=${PROXY}
}

getLastNodeId() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getLastNodeId" --arguments ${CALLER_ADDRESS_HEX}
}

getNodeIds() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getNodeIds" --arguments ${CALLER_ADDRESS_HEX}
}

getNodesPerStaker() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getNodesPerStaker" --arguments ${CALLER_ADDRESS_HEX}
}

getNode() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getNode" --arguments ${CALLER_ADDRESS_HEX} 1
}

getStakeTokenId() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getStakeTokenId"
}

getRewardTokenId() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getRewardTokenId"
}

getReferrerAddress() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getReferrerAddress" --arguments ${CALLER_ADDRESS_HEX}
}

getReferralActivated() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getReferralActivated" --arguments ${CALLER_ADDRESS_HEX}
}

getReferredCount() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getReferredCount" --arguments ${CALLER_ADDRESS_HEX}
}

getReferralActivationAmount() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getReferralActivationAmount"
}

getApyIncreasePerReferral() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getApyIncreasePerReferral"
}

getMaxApyIncreaseByReferral() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getMaxApyIncreaseByReferral"
}

getReferralReward() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getReferralReward"
}

getApyOfStaker() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getApyOfStaker" --arguments ${CALLER_ADDRESS_HEX} 1
}