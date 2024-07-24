# Boomerang Canister

The Boomerang canister has been thought to allow any form of decentralized governance to use the WaterNeuron protocol without relying on a centralized party to actually do the operation. We still recommand single individuals to use the DApp as it remains the simplest way to use the protocol. 

For our usecase, we will be taking the example of an SNS DAO which is the prevalent type of organisation within the Internet Computer Community. 

Converting ICP from the treasury funds of the SNS is pretty straightforward. 


```
┌───┐                  ┌─────────┐                        ┌──────────┐┌───────────┐┌───────────┐┌──────────┐
│SNS│                  │Boomerang│                        │ICP ledger││WaterNeuron││nICP ledger││WTN ledger│
└─┬─┘                  └────┬────┘                        └────┬─────┘└─────┬─────┘└─────┬─────┘└────┬─────┘
  │                         │                                  │            │            │           │      
  │get_account_id(principal)│                                  │            │            │           │      
  │────────────────────────>│                                  │            │            │           │      
  │                         │                                  │            │            │           │      
  │        AccountID        │                                  │            │            │           │      
  │<────────────────────────│                                  │            │            │           │      
  │                         │                                  │            │            │           │      
  │          (*) TransferSnsTreasuryFunds(AccountID)           │            │            │           │      
  │───────────────────────────────────────────────────────────>│            │            │           │      `
  │                         │                                  │            │            │           │      
  │                         Received!                          │            │            │           │      
  │<───────────────────────────────────────────────────────────│            │            │           │      
  │                         │                                  │            │            │           │      
  │    notify_transfer()    │                                  │            │            │           │      
  │────────────────────────>│                                  │            │            │           │      
  │                         │                                  │            │            │           │      
  │                         │   icrc1_balance_of(principal)    │            │            │           │      
  │                         │─────────────────────────────────>│            │            │           │      
  │                         │                                  │            │            │           │      
  │                         │           Some(amount)           │            │            │           │      
  │                         │<─────────────────────────────────│            │            │           │      
  │                         │                                  │            │            │           │      
  │                         │icrc2_approve(WaterNeuron, amount)│            │            │           │      
  │                         │─────────────────────────────────>│            │            │           │      
  │                         │                                  │            │            │           │      
  │                         │                 icp_to_nicp()    │            │            │           │      
  │                         │──────────────────────────────────────────────>│            │           │      
  │                         │                                  │            │            │           │      
  │                         │                  Converted!      │            │            │           │      
  │                         │<──────────────────────────────────────────────│            │           │      
  │                         │                                  │            │            │           │      
  │                         │            icrc1_transfer(to SNS, nicp_amount)│            │           │      
  │                         │───────────────────────────────────────────────────────────>│           │      
  │                         │                                  │            │            │           │      
  │                         │                    icrc1_transfer(to SNS, airdrop)         │           │      
  │                         │───────────────────────────────────────────────────────────────────────>│      
┌─┴─┐                  ┌────┴────┐                        ┌────┴─────┐┌─────┴─────┐┌─────┴─────┐┌────┴─────┐
│SNS│                  │Boomerang│                        │ICP ledger││WaterNeuron││nICP ledger││WTN ledger│
└───┘                  └─────────┘                        └──────────┘└───────────┘└───────────┘└──────────┘
```

```
┌───┐                                                                                                        ┌───┐
│SNS│                                                                                                        │DAO│
└─┬─┘                                                                                                        └─┬─┘
  │                                                                                                            │  
  │(*) AddGenericNervousSystemFunction(target_canister nicp_ledger, method icrc1_transfer(to Boomerang, amount)│  
  │───────────────────────────────────────────────────────────────────────────────────────────────────────────>│  
┌─┴─┐                                                                                                        ┌─┴─┐
│SNS│                                                                                                        │DAO│
└───┘                                                                                                        └───┘


┌───┐                                 ┌───────────┐                       ┌─────────┐  ┌───────────┐┌──────────┐
│SNS│                                 │nICP ledger│                       │Boomerang│  │WaterNeuron││ICP ledger│
└─┬─┘                                 └─────┬─────┘                       └────┬────┘  └─────┬─────┘└────┬─────┘
  │                                         │                                  │             │           │      
  │(*) ExecuteGenericNervousSystemFunction()│                                  │             │           │      
  │────────────────────────────────────────>│                                  │             │           │      
  │                                         │                                  │             │           │      
  │                Received!                │                                  │             │           │      
  │<────────────────────────────────────────│                                  │             │           │      
  │                                         │                                  │             │           │      
  │                           notify_nicp_transfer()                           │             │           │      
  │───────────────────────────────────────────────────────────────────────────>│             │           │      
  │                                         │                                  │             │           │      
  │                                         │   icrc1_balance_of(Boomerang)    │             │           │      
  │                                         │<─────────────────────────────────│             │           │      
  │                                         │                                  │             │           │      
  │                                         │           Some(amount)           │             │           │      
  │                                         │─────────────────────────────────>│             │           │      
  │                                         │                                  │             │           │      
  │                                         │icrc2_approve(WaterNeuron, amount)│             │           │      
  │                                         │<─────────────────────────────────│             │           │      
  │                                         │                                  │             │           │      
  │                                         │                                  │nicp_to_icp()│           │      
  │                                         │                                  │────────────>│           │      
  │                                         │                                  │             │           │      
  │                                         │                                  │ Converted!  │           │      
  │                                         │                                  │<────────────│           │      
  │                                         │                                  │             │           │      
  │                                         │                                  │transfer(to SNS, amount) │      
  │                                         │                                  │────────────────────────>│      
┌─┴─┐                                 ┌─────┴─────┐                       ┌────┴────┐  ┌─────┴─────┐┌────┴─────┐
│SNS│                                 │nICP ledger│                       │Boomerang│  │WaterNeuron││ICP ledger│
└───┘                                 └───────────┘                       └─────────┘  └───────────┘└──────────┘
```