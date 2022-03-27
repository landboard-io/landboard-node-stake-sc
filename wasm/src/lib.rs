////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    landboard_node_stake
    (
        addStakeTypes
        changeStakeType
        claim
        clearStakeTypes
        disableStakeTypes
        enableStakeTypes
        getActivatedReferrerAddresses
        getApyIncreasePerReferral
        getApyOfStaker
        getLastNodeId
        getMaxApyIncreaseByReferral
        getNode
        getNodeIds
        getNodesPerStaker
        getPaused
        getPromoIncreaseApy
        getReferralActivated
        getReferralActivationAmount
        getReferredCount
        getReferrerAddress
        getRewardTokenId
        getStakeTokenId
        getStakeTypes
        getStakerAddresses
        getTotalReferralCount
        getTotalReferralCountLimit
        pause
        setApyIncreasePerReferral
        setMaxApyIncreaseByReferral
        setPromoIncreaseApy
        setReferralActivationAmount
        setRewardTokenId
        setStakeTokenId
        setTotalReferralCountLimit
        stake
        unpause
        unstake
        withdraw
    )
}

elrond_wasm_node::wasm_empty_callback! {}
