##### - configuration - #####
PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"

WALLET="./wallets/test-wallet.pem"
WALLET2="./wallets/test-wallet-2.pem"

TOKEN_ID="SVEN-4b35b0"
TOKEN_ID_HEX="0x$(echo -n ${TOKEN_ID} | xxd -p -u | tr -d '\n')"



#####
ADDRESS=$(erdpy data load --key=address-devnet)
TRANSACTION=$(erdpy data load --key=deployTransaction-devnet)
######

deploy() {
    erdpy --verbose contract deploy \
    --project=${PROJECT} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=50000000 \
    --arguments ${TOKEN_ID_HEX} ${TOKEN_ID_HEX} \
    --send \
    --metadata-payable \
    --outfile="deploy-devnet.interaction.json" \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} || return

    TRANSACTION=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['address']")

    erdpy data store --key=address-devnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-devnet --value=${TRANSACTION}
}

makeOffer() {
    erdpy --verbose tx new --receiver ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --data="ESDTTransfer@${OFFER_TOKEN_ID_ONLY_HEX}@${OFFER_TOKEN_AMOUNT_ONLY_HEX}@${MAKE_OFFER_ONLY_HEX}@${ACCEPT_TOKEN_ID_ONLY_HEX}@${ACCEPT_TOKEN_AMOUNT_ONLY_HEX}" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

cancelOffer() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="cancelOffer" \
    --argument 1 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

OFFER_TOKEN_ID_HEX="0x$(echo -n ${OFFER_TOKEN_ID} | xxd -p -u | tr -d '\n')"

acceptOffer() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET2} \
    --gas-limit=6000000 \
    --function="acceptOffer" \
    --value ${ACCEPT_TOKEN_AMOUNT} \
    --argument 1 ${OFFER_TOKEN_ID_HEX} ${OFFER_TOKEN_AMOUNT} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

withdraw() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="withdraw" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

# config

getOfferIds() {
    erdpy --verbose contract query ${ADDRESS} --function="getOfferIds" --proxy=${PROXY}
}

getOffers() {
    erdpy --verbose contract query ${ADDRESS} --function="getOffers" --proxy=${PROXY}
}

getOffer() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getOffer" --argument 1
}