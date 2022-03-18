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
        getReferralActivated
        getReferralActivationAmount
        getReferredCount
        getReferrerAddress
        getRewardTokenId
        getStakeTokenId
        getStakeTypes
        getStakerAddresses
        pause
        setApyIncreasePerReferral
        setMaxApyIncreaseByReferral
        setReferralActivationAmount
        setRewardTokenId
        setStakeTokenId
        stake
        unpause
        unstake
        withdraw
    )
}

elrond_wasm_node::wasm_empty_callback! {}
