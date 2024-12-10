use std::cmp::Ordering;
use std::fmt::{Display};
use std::ops::{Div};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum NumericBlockType {
    Space,
    Data(u32)
}

impl Display for NumericBlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumericBlockType::Space => {write!(f, ".")}
            NumericBlockType::Data(k) => {write!(f, "({})", k)}
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct Block {
    pub(crate) block_type: NumericBlockType,
    pub(crate) start: u32,
    pub(crate) length: u32
}

pub(crate) struct Day09 {
    pub(crate) data_blocks: Vec<Block>,
    pub(crate) space_blocks: Vec<Block>,
    pub (crate) disk: Vec<NumericBlockType>,
}

impl Day09 {
    pub(crate) fn new(input: &str) -> Day09 {
        let mut data_blocks: Vec<Block> = Vec::new();
        let mut space_blocks: Vec<Block> = Vec::new();
        let mut offset: u32 = 0;

        for (i, chunk) in input.trim().chars().enumerate() {
            let n: u32 = chunk.to_digit(10).unwrap();

            if i%2 == 0 {
                data_blocks.push(Block{
                    block_type: NumericBlockType::Data(i.div(2) as u32),
                    start: offset,
                    length: n,
                })
            } else {
                space_blocks.push(Block{
                    block_type: NumericBlockType::Space,
                    start: offset,
                    length: n,
                })
            }

            offset += n;
        }

        let missing = data_blocks.len() - space_blocks.len();
        for _ in 0..missing {
            space_blocks.push(Block {
                block_type: NumericBlockType::Space,
                start: offset,
                length: 0
            })
        }

        let mut disk: Vec<NumericBlockType> = Vec::new();
        for i in 0..data_blocks.len() {
            for _ in 0..data_blocks[i].length {
                disk.push(data_blocks[i].block_type);
            }

            for _ in 0..space_blocks[i].length {
                disk.push(space_blocks[i].block_type);
            }
        }

        Day09{
            data_blocks,
            space_blocks,
            disk,
        }
    }
}

fn render_disk(disk: &[NumericBlockType]) -> String {
    disk.iter().map(|entry| {
        match entry {
            NumericBlockType::Space => ".".to_string(),
            NumericBlockType::Data(k) => {
                format!("-{}-", k)
            }
        }
    }).collect::<String>().to_string()
}
pub(crate) fn solve(data: &str) -> u64 {
    let day = Day09::new(data);

    let mut data_reversed = day.data_blocks.clone();
    data_reversed.reverse();
    data_reversed.pop().unwrap();

    let mut whoops_all_data:  Vec<Block> = Vec::new();
    // push the first element of the vec because that's not moving
    let mut block_idx = 0;
    let mut block_to_move = data_reversed[block_idx];
    let mut total_moved = 0;

    for (i, space) in day.space_blocks.iter().enumerate() {
        // println!("\n\
        // --------------------\n\
        // looping on space idx {}\n\
        // --------------------", i);

        if let NumericBlockType::Data(k) = block_to_move.block_type  {
            // println!("checking the block id {} from the reversed to the block we're moving: {}", k, i);
            if k == i as u32 {
                println!("reached the same data block from both sides at {}, breaking", k);
                break;
            }

            // println!("processing data block from the back {} while going from the front {}", k, i);
        }

        whoops_all_data.push(day.data_blocks[i]);
        // println!("pushed data block at {} into all data vec", i);
        // println!("all data vec: {:?}", whoops_all_data);

        total_moved = space.start;
        // println!("set total moved to {}", total_moved);

        let mut remaining = space.length;
        // println!("remaining: {}", remaining);

        while  remaining > 0 {
            // println!("\n -- starting a while loop as remaining is {}", remaining);
            if block_to_move.length == 0 {
                block_idx += 1;
                block_to_move = data_reversed[block_idx];

                // println!("empty data block, advancing block idx to {} and remaining in block to {}", block_idx, block_to_move.length);
            }

            match remaining.cmp(&block_to_move.length) {
                Ordering::Less => {
                    // println!("remaining in space {} was less than remaining in block {}", remaining, block_to_move.length);

                    whoops_all_data.push(Block{
                        block_type: block_to_move.block_type, // same type as the block
                        length: remaining,
                        start: total_moved,
                    });

                    total_moved += remaining;

                    block_to_move.length -= remaining;
                    remaining = 0;
                    // println!("pushed a new block to whoops all data. Total moved is {}, remaining set to 0, block length left is {}.New vec\n{:?}\n", total_moved, block_to_move.length, whoops_all_data)
                }
                Ordering::Equal => {
                    // println!("remaining in space {} was equal to remaining in block {}", remaining, block_to_move.length);
                    block_to_move.start = total_moved;

                    whoops_all_data.push(block_to_move);
                    block_to_move.length = 0;

                    total_moved += remaining;
                    remaining = 0;

                    // println!("pushed the entire block to whoops all data, remaining is zero, total moved is {}. New vec\n{:?}\n", total_moved, whoops_all_data)
                }
                Ordering::Greater => {
                    // println!("remaining in space {} was greater than remaining in block {}", remaining, block_to_move.length);
                    let mut to_move = data_reversed[block_idx];
                    to_move.start = total_moved;
                    whoops_all_data.push(to_move);

                    // println!("pushed the entire block to the whoops vec. New vec is\n{:?}\n", whoops_all_data);

                    total_moved += block_to_move.length;

                    // lower the amount of space the next block can take up
                    remaining -= block_to_move.length;
                    // mark remaining spaces in the data block that need to be put into the next
                    // space as zero
                    block_to_move.length = 0;
                    // println!("set total moved to {}, remaining to {}, and block to move to 0", total_moved, remaining);
                }
            }
        }
    }

    if block_to_move.length > 0 {
        block_to_move.start = total_moved;
        // println!("we had more left, so pushing those into the block to move: {:?}", block_to_move);

        whoops_all_data.push(block_to_move);
    }

    // println!("final all data vec is\n{:?}\n", whoops_all_data);

    let mut disk: Vec<NumericBlockType> = Vec::new();
    for data in whoops_all_data {
        for _ in 0..data.length {
            disk.push(data.block_type);
        }
    }

    disk_checksum(&*disk)
}


pub(crate) fn solve_swap(data: &str) -> u64 {
    let day = Day09::new(data);
    let mut disk_copy = day.disk.clone();

    let mut next_space_idx = 0;
    let mut next_block_idx = disk_copy.len()-1;

    while next_space_idx < next_block_idx {
        match disk_copy[next_space_idx] {
            NumericBlockType::Space => {
                // if we have a space idx, check for next block from the back
                match disk_copy[next_block_idx] {
                    NumericBlockType::Space => {
                        // we're looking for a block, found space, so keep moving
                        next_block_idx -=1;
                    }
                    NumericBlockType::Data(_) => {
                        disk_copy.swap(next_space_idx, next_block_idx);
                        next_space_idx += 1;
                        next_block_idx -= 1;
                    }
                }
            }
            NumericBlockType::Data(_) => { next_space_idx += 1; }
        }
    }

    disk_checksum(&disk_copy)
}

pub(crate) fn disk_checksum(disk: &[NumericBlockType]) -> u64 {
    let mut sum = 0;
    for (i, block) in disk.iter().enumerate() {
        match block {
            NumericBlockType::Space => {}
            NumericBlockType::Data(k) => {
                sum += (i as u64) * (*k as u64);
            }
        }
    }

    sum
}
