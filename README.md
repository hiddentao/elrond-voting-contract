# Elrond voting contract demo

This is a simple voting contract written in Rust for the [Elrond VM](https://elrond.com).

It lets callers choose between 2 options and uses uses a [commit-reveal](https://karl.tech/learning-solidity-part-2-voting/) mechanism to ensure vote privacy.

The flow:

1. Contract is deployed with with the maximum no. of users who can vote set.
1. Users start voting by calling `commit()` with a hashed version of their vote, specifically `keccak256(<vote>|<salt>)` - `<vote>`:
  * `<vote>` is either `0` or `1`
  * `<salt>` is a random keccak256 hash
1. Until the max. no. of users have voted, each user is allowed to overwrite their previous vote commitment with a new one.
1. Once the max no. of users have cast votes, the _reveal_ phase beings.
1. Users now reveal their votes by calling `reveal()`, passing in their `<vote>` and `<salt>`.
1. The contract will hash these parameters and compare the result to the stored commitment hash from earlier. If they match then the user's vote counts towards the final vote tallies. If they mismatch the contract will throw an error.
  * _Note: a user can only successfully call `reveal()` once_
1. The final vote tallies can be obtained by calling `getVote1Tally()` and `getVote2Tally()`.

## Dev guide

Pre-requisites:

* [erdpy 0.5.4b1](https://pypi.org/project/erdpy/#history)

Clone the repo, and then inside the folder run:

```shell
erdpy contract build .
```

Now run the tests:

```shell
erdpy contract test .
```

To run an individual test (e.g. `basic.scen.json`):

```shell
mandos-test "./test/basic.scen.json"
```

## License

Copyright 2020 Ramesh Nair

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.