# DOT NOT do IT

Please do not do it !

assert!() can not revert state updated before any outside call.

```

  dfx deploy  --no-wallet      

  dfx canister call standard raiseError "error code 123123,and the error details is xxxxxx"

  dfx canister call standard getValue  // (1)

  dfx canister call stardard getValue2 // (0)
```

# Dfinity Call Error Standard

## Overview

Canister update call:
  + you can revert state update by throw an exception,  that means the error message cannot be returned. 
  + If you want to return a error message , the state updated cannot be revert automatically. 

 The returned error message can help developers locate the problem and find a solution quickly. There should be a new error return method to balance these problems.

 ## Solution

The update call only needs to design the return value under the correct execution. If an exception occurs, Canister returns the response error message through the getLastError method to help the developer locate the problem.

```
service:{
  getLastError: ()->(text) query;
}
```

## Example 
   You can test it with the following command :

   ```
      dfx deploy  --no-wallet      

      dfx canister call standard raiseError "error code 123123,and the error details is xxxxxx"

      dfx canister call standard getLastError
   ```

## About us

   We are from Deland-Labs team. 

   We are building a decentralized exchange based on Dfinity with Open Order Protocol.

   Offcial Website : [https://deland.one](https://deland.one)