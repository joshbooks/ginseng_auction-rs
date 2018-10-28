pub use self::guest::Guest;
pub use self::guest::Range;

mod guest
{
    use std::cmp::Ordering;
    use std::vec::Vec;

    #[derive(Debug, Clone, Eq)]
    pub struct Range
    {
        pub min: u64,
        pub max: u64,
    }

    // impl Range
    // {
    //     pub fn min(&self) -> u64
    //     {
    //         self.min
    //     }
    //
    //     pub fn max(&self) -> u64
    //     {
    //         self.max
    //     }
    // }

    impl Ord for Range {
        // non-overlapping so it doesn't matter which we compare
        // as long as we don't compare different guests' ranges with
        // each other. If we need to do that we'll need to reimagine
        // this comparison in a way that's appropriate to the
        // particular use case
        fn cmp(&self, other: &Range) -> Ordering {
            self.min.cmp(&other.min)
        }
    }

    impl PartialOrd for Range {
        fn partial_cmp(&self, other: &Range) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for Range {
        fn eq(&self, other: &Range) -> bool {
            self.min == other.min
        }
    }

    #[derive(Debug, Eq)]
    pub struct Guest
    {
        //first picks a unit of memory, let's say kibibytes
        //then we pick a unit of time, let's say minutes
        // (calculating the knapsack problem can take up
        // to a second so probs not seconds). Then a unit
        // of money, let' ssay united states cents. In this
        // example we would then have

        // the last bid of cents per kibibyte-minute
        pub mem_unit_price: u64,
        // how many kibibytes the Guest currently holds as the result of previous bids
        pub current_holdings: u64,
        // what minimum number of kibibytes to what maximum number of kibibytes
        // would be a useful addition to the guest in question (below point of diminishing
        // returns, but large enough to make a difference type thing)
        // should be non-overlapping and sorted in ascending order

        // todo create a constructor that sorts on instantiation so we
        // don't have to worry about it
        pub forbidden_ranges: Vec<Range>,
        pub base_memory: u64,
    }

    // impl Guest
    // {
    //     pub fn forbidden_ranges(&self) -> &Vec<Range>
    //     {
    //         &self.forbidden_ranges
    //     }
    //
    //     pub fn mem_unit_price(&self) -> u64
    //     {
    //         self.mem_unit_price
    //     }
    // }

    impl Ord for Guest {
        fn cmp(&self, other: &Guest) -> Ordering {
            other.mem_unit_price
                .cmp(&self.mem_unit_price)
                .then(self.current_holdings.cmp(&other.current_holdings))
                //by using sort_unstable we effectively get tie breaking
                // random ordering as specified in the paper, but without
                // making eq and cmp disagree with each other
        }
    }

    impl PartialOrd for Guest {
        fn partial_cmp(&self, other: &Guest) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for Guest {
        fn eq(&self, other: &Guest) -> bool {
            other.mem_unit_price == self.mem_unit_price &&
                self.current_holdings == other.current_holdings
        }
    }
}
