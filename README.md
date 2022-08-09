# EVM Mneumonic

Inspired by [EVM codes](https://www.evm.codes/playground?unit=Wei&codeType=Mnemonic&code='z0x4wMSTORE~3wRETURN'~yzzPUSH1%20y%5Cnw2~0y%01wyz~_), EVM Mneumonic (EVMM) is a simple framework for writing barebones, hand tuned smart contracts.


## Installing


## Syntax
(Will update later with a real snippet for swapping on a DEX or something.)

```rust
PUSH1 0x42
PUSH1 0
MSTORE
PUSH1 32
PUSH1 0
RETURN
PUSH1 0x42
PUSH1 0
MSTORE
PUSH1 32
PUSH1 0
RETURN
```


## Compiling


## Testing


## EVMM Repl

## TODOS:
(Will make issues for each todo)

- CI Pipeline.
- More human readable error messages.
- Simple benchmark comparison in Foundry between Solidity, Vyper, Yul, Huff, EVMM.
- Thorough snapshot parser tests