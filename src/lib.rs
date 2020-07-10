#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

imports!();

#[elrond_wasm_derive::contract(VoteImpl)]
pub trait Vote {
    // max votes
    #[private]
    #[storage_set("maxVotes")]
    fn _setMaxVotes(&self, maxVotes: u16);

    #[storage_get("maxVotes")]
    fn getMaxVotes(&self) -> u16;

    // votes so far
    #[private]
    #[storage_set("voteCount")]
    fn _setVoteCount(&self, count: u16);

    #[storage_get("voteCount")]
    fn getVoteCount(&self) -> u16;

    // votes revealed so far
    #[private]
    #[storage_set("vote1Tally")]
    fn _setVote1Tally(&self, count: u16);

    #[storage_get("vote1Tally")]
    fn getVote1Tally(&self) -> u16;

    #[private]
    #[storage_set("vote2Tally")]
    fn _setVote2Tally(&self, count: u16);

    #[storage_get("vote2Tally")]
    fn getVote2Tally(&self) -> u16;

    // vote commitments
    #[private]
    #[storage_set("voteCommitment")]
    fn _setVoteCommitment(&self, voter: &Address, value: H256);

    #[storage_get("voteCommitment")]
    fn getVoteCommitment(&self, voter: &Address) -> H256;

    // vote reveals
    #[private]
    #[storage_set("voteReveal")]
    fn _setVoteReveal(&self, voter: &Address, value: u8);

    #[storage_get("voteReveal")]
    fn getVoteReveal(&self, voter: &Address) -> u8;

    // constructor
    fn init(&self, maxVotes: u16) {
        self._setMaxVotes(maxVotes);
    }

    fn allVotesCast(&self) -> bool {
        let voteCount = self.getVoteCount();
        let maxVotes = self.getMaxVotes();
        voteCount == maxVotes
    }

    fn allVotesRevealed(&self) -> bool {
        let voteRevealCount = self.getVote1Tally() + self.getVote2Tally();
        let maxVotes = self.getMaxVotes();
        voteRevealCount == maxVotes
    }

    // commit vote
    fn commit(&self, value: H256) -> Result<(), &str> {
        // check that a vote can still be cast
        let allVotesCast = self.allVotesCast();
        if allVotesCast {
            return Err("no more votes available");
        }

        // save vote
        self._setVoteCommitment(&self.get_caller(), value);
        self._setVoteCount(self.getVoteCount() + 1);

        Ok(())
    }

    // reveal vote
    fn reveal(&self, vote: u8, salt: H256) -> Result<(), &str> {
        // check that all votes have been cast
        let allVotesCast = self.allVotesCast();
        if !allVotesCast {
            return Err("not all votes have been cast yet")
        }

        let voter = self.get_caller();

        // check that caller has previously voted a commitment
        let voteCommitment = self.getVoteCommitment(&voter);
        if voteCommitment == H256::zero()  {
            return Err("not a voter");
        }

        // check that caller has not already revealed their vote
        let voteReveal = self.getVoteReveal(&voter);
        if 0 < voteReveal {
            return Err("already revealed vote");
        }

        // calculate expected commitment
        let mut raw_key: Vec<u8> = Vec::with_capacity(33);
        raw_key.push(vote);
        raw_key.extend_from_slice(salt.as_bytes());
        let key = self.keccak256(&raw_key);
        let expectedCommitment = H256::from_slice(&key);

        // check that it matches the stored commitment
        if expectedCommitment != voteCommitment {
            return Err("vote mismatch");
        }

        // save the revealed vote
        self._setVoteReveal(&voter, vote);

        // inc tally
        if vote == 1 {
            self._setVote1Tally(self.getVote1Tally() + 1)
        } else {
            self._setVote2Tally(self.getVote2Tally() + 1)
        }

        Ok(())
    }
}
