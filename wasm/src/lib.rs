////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    landboard_node_stake
    (
        addStakeTypes
        claim
        clearStakeTypes
        getApyIncreasePerReferral
        getLastNodeId
        getMaxApyIncreaseByReferral
        getNode
        getNodeIds
        getNodesPerStaker
        getPaused
        getPromoIncreaseApy
        getReferralActivated
        getReferralActivationAmount
        getReferralReward
        getReferredCount
        getReferrerAddress
        getRewardTokenId
        getStakeTokenId
        getStakeTypes
        getStakerAddresses
        pause
        setApyIncreasePerReferral
        setMaxApyIncreaseByReferral
        setPromoIncreaseApy
        setReferralActivationAmount
        setReferralReward
        setRewardTokenId
        setStakeTokenId
        stake
        unpause
        unstake
        withdraw
    )
}

elrond_wasm_node::wasm_empty_callback! {}
