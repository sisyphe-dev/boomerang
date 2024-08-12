```
dfx canister call wtn_governance manage_neuron '(record {
    subaccount = blob "${NEURON_ID}"; 
    command = opt variant {
        MakeProposal = record {
            url = "https://docs.waterneuron.fi/"; 
            title = "Stake ICP Treasury with WaterNeuron";
            summary = "
This proposal to stake ICP treasury with WaterNeuron.
            "; 
            action = opt variant { 
                TransferSnsTreasuryFunds = record { 
                    from_treasury = 1;
                    to_principal = opt principal "daijl-2yaaa-aaaar-qag3a-cai";
                    to_subaccount = opt record {
                        subaccount = blob "${DAO_SUBACCOUNT}";
                    };
                    memo = null;
                    amount_e8s = $AMOUNT;
                }
            }
        }
    }
})' --network ic
```