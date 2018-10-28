extern crate ginseng;

use ginseng::guest::Guest;
use ginseng::guest::Range;

use std::cmp::min;
use std::cmp::Ordering;
use std:collections::HashMap;
use std::vec::Vec;


fn social_welfare(proposed_allocation: &Vec<(&Guest, u64)>) -> u64
{
    let mut total_welfare: u64 = 0;

    for (guest, allocation) in proposed_allocation
    {
        total_welfare += guest.mem_unit_price * allocation;
    }

    total_welfare
}

// assumes that `guest_list` has already been sorted
// just doles out memory in sorted order such that we
// don't give anyone any more memory than they asked for
// it's still possible that we give people too little memory
// which should be checked after getting the result of this
// function

// I think what I really want to return from this is a vector of allocation amounts
// it'll even take up less space than (reference, allocation) pairs and will be much
// less problematic
fn naive_allocation
(
    guest_list: &Vec<&Guest>,
    available_memory: u64
)
-> Vec<(u64)>
{
    let mut remaining_memory: u64 = available_memory;

    guest_list.iter().map
    (
        |guest|
        {
            // if there's no memory left to hand out our job is simple
            if remaining_memory == 0
            {
                0
            }
            // otherwise get the maximum amount memory this guest
            // wants to pay for and give them that
            else
            {
                // if the last forbidden range goes to infinity we want
                //to use the minimum of that forbidden range
                let upper_bound =
                    guest
                        .forbidden_ranges
                        .last()
                        .filter(|range| {range.max == u64::max_value()})
                        .map(|range| {range.min})
                        .unwrap_or(u64::max_value());

                let mem_to_alloc = min(remaining_memory, upper_bound);

                remaining_memory -= mem_to_alloc;

                mem_to_alloc
            }
        }
    ).collect()
}

// this guy already returns indices instead of references
fn invalid_allocations(proposed_allocations: &Vec<(&Guest, u64)>) -> Vec<(usize, Range)>
{
    let mut violations: Vec<(usize, Range)> = Vec::new();
    let mut index: usize = 0;

    for &(guest, amount) in proposed_allocations.iter()
    {
        // we want to get the guest's forbidden range with the greatest
        // min value that is less than amount. This would probably best
        // be done with a binary search, but for now iterative is fine
        for range in guest.forbidden_ranges.clone()
        {
            if range.min < amount && range.max > amount
            {
                violations.push((index, range));
            }
        }
        index = index + 1;
    }

    violations
}

fn public_auction_function
(
    guest_list: &Vec<&Guest>,
    available_memory: u64
)
-> Vec<u64>
{
    auction_with_pinned_allocations(guest_list, available_memory, Vec::new())
}

// returns the list of allocations of the provided memory to the list of
// provided guests which results in the maximal social welfare possible without
// changing the provided pinned allocations
fn auction_with_pinned_allocations
(
    guest_list: &Vec<&Guest>,
    available_memory: u64,
    pinned_allocations: &HashMap<u64, u64> //list of (index, allocation) pairs
)
-> Vec<u64>
{
    // so I think the idea is we filter the pinned allocations out in an
    // enumerate into a filter into a map that discards the index into a
    // collect

    // then we put them back into the result of the recursive case with an
    // enumerate into a flatmap that returns element +
    // contiguous-succesive-elements. So we just need a mutable `index_correction`
    // variable that gets incremented every time we insert an element so we
    // can keep track of each elements place in the new list so we can map positions in
    // the enumerate to positions in the original list

    let my_copy =
        guest_list
            .iter()
            .enumerate()
            .filter(|(index, &guest)| !pinned_allocations.contains_key(index))
            .map(|(index, &guest)| guest.clone())
            .collect()


    // let mut my_copy = guest_list.clone();

    let invalid = invalid_allocations(my_copy.iter().zip(naive_allocation(my_copy, available_memory))





}

// Given the applicable range for an invalid allocation and the amount of available_memory
// this function returns the one to two allocation options available to replace the
// invalid allocation
fn two_options(applicable_range: Range, available_memory: u64) -> Vec<u64>
{
    match applicable_range.max.cmp(&available_memory)
    {
        Ordering::Less | Ordering::Equal =>
            vec![applicable_range.min, applicable_range.max],

        Ordering::Greater => vec![applicable_range.min],
    }
}

//I think I want an immutable list slice that I jut progressively slice more off of
// or present different iterators of maybe?
// the payment rule should be implemented in a different function
// so that we can use this function recursively

// I think what we should really return from this is a list of (index, allocation) pairs
// and take as input a list of inidices of invalid allocations (we can have a helper function
//without that argument that passes an empty list into this function)
// fn auction<'a>(guest_list: &'a mut Vec<&'a Guest>, available_memory: u64) -> Vec<(&'a Guest, u64)>
// {
//     if guest_list.is_empty()
//     {
//         return Vec::new();
//     }
//
//     // so first we try just try giving as much as
//     // possible of the remaining_memory
//     // to each guest
//     let applicable_range: Range;
//     let invalid_guest_number;
//     {
//         //then we first attempt a naive allocation based solely on the sorting
//         //and any upper bounds the guests may have set
//         // naive_allocation should maintain the same ordering of geusts as guest_list
//         // so indices of proposed_allocation can be used to index into guest_list
//         let proposed_allocations = naive_allocation(guest_list, available_memory);
//
//         let mut invalid = invalid_allocations(&proposed_allocations);
//
//         // yup, commenting out this early return got rid of two of the 3 compile errors
//         // and the third is pretty obviously a good thing to error out on because it let me know
//         //I was adding two guests for case two instead of just one
//         // if invalid.is_empty()
//         // {
//         //     return proposed_allocations;
//         // }
//
//         // so with this first attempt we want to first check and see if we're
//         // assigning someone an amount of memory in one of their forbidden ranges
//         // and for each case in which someone was allocated an invalid amount, we
//         // need to try two cases.
//         // so we just need to try removing the first invalid allocation, which means
//         // we can just mutate the guest_list instead of cloning every time
//         let (_invalid_guest_number, _applicable_range) = invalid.remove(0);
//         invalid_guest_number = _invalid_guest_number;
//         applicable_range = _applicable_range;
//
//     }
//     //so we remove the first invalid allcoation
//     let badly_served_guest = guest_list.remove(invalid_guest_number);
//
//     // and then we try the two cases with that guest
//
//     // So I think the idea is that we try the minimum and maximum of the
//     // forbidden range that the invalid value fell into
//
//     //case one is no more than the minimum of the forbidden range
//     let allocation_amount_one = applicable_range.min;
//
//     let mut case_one_proposal = auction(guest_list, available_memory - allocation_amount_one);
//
//     case_one_proposal.push((badly_served_guest, allocation_amount_one));
//
//     let case_one_welfare = social_welfare(&case_one_proposal);
//
//     //case two is at least as much as the maximum of the forbidden range
//     let allocation_amount_two = applicable_range.max;
//
//     let (case_two_welfare, case_two_proposal) =
//         if allocation_amount_two <= available_memory
//         {
//             let mut inner_case_two_proposal =
//                 auction(guest_list, available_memory - allocation_amount_two);
//
//             inner_case_two_proposal.push((badly_served_guest, allocation_amount_two));
//
//             (social_welfare(&inner_case_two_proposal), inner_case_two_proposal)
//         }
//         else
//         {
//             (0, Vec::new())
//         };
//
//
//
//     //return the one with greater welfare, or if equal, the one that allocates less memory
//     match case_one_welfare.cmp(&case_two_welfare)
//     {
//         Ordering::Less => case_two_proposal,
//
//         Ordering::Greater => case_one_proposal,
//
//         Ordering::Equal => case_one_proposal,
//     }
// }

// fn registerGuest(baseMemory: i64)
// {
//
// }

// fn makeBid(mem_unit_price: f64, guest: Guest)
// {
//
// }

fn main()
{
    let guest1 =
        Guest
        {
            mem_unit_price: 2,
            current_holdings: 1,
            forbidden_ranges:
                vec!
                [
                    Range{min: 0, max: 3},
                    Range{min: 4, max: u64::max_value()}
                ],
            base_memory: 10
        };

    let guest2 =
        Guest
        {
            mem_unit_price: 1,
            current_holdings: 1,
            forbidden_ranges:
                vec!
                [
                    Range{min: 0, max: 3},
                    Range{min: 5, max: u64::max_value()}
                ],
            base_memory: 10
        };

    let mut guest_list = vec![&guest1, &guest2];

    guest_list.sort_unstable();

    {
        for guest in &guest_list
        {
            println!("{:?}", guest);
        }
    }

    {
        let naive = naive_allocation(&guest_list, 6);


        println!("The naive allocation is: ", );
        {
            for (ref guest, allocated) in &naive
            {
                println!("{:?} gets {:?}", guest, allocated);
            }
        }
        println!("it has a social welfare of {:?}", social_welfare(&naive));
    }

    let final_allocation = auction(&mut guest_list, 6);

    println!("The final allocation is: ", );
    {
        for (ref guest, allocated) in &final_allocation
        {
            println!("{:?} gets {:?}", guest, allocated);
        }
    }
    println!("it has a social welfare of {:?}", social_welfare(&final_allocation));

    println!("Hello, world!");
}
