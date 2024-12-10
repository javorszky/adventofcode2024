use crate::part1::{disk_checksum, Block, NumericBlockType};

pub(crate) fn solve(data: &str) -> u64 {
    let day = crate::part1::Day09::new(data);

    let mut all_blocks: Vec<Block> = Vec::new();
    for (i, space_block) in day.space_blocks.iter().enumerate() {
        all_blocks.push(day.data_blocks[i]);
        all_blocks.push(*space_block);
    }

    // println!("all blocks: {:?}", all_blocks);

    compact_full_files(&mut all_blocks);

    let mut disk: Vec<NumericBlockType> = Vec::new();
    for data in all_blocks {
        for _ in 0..data.length {
            disk.push(data.block_type);
        }
    }

    // println!("this is disk: {:?}", disk);
    disk_checksum(&*disk)
}

fn compact_full_files(blocks: &mut Vec<Block>) {
    let last_idx = blocks.len()-1;
    let mut last_id: u32 = 0;

    let last_block_type = blocks.get(last_idx-1).unwrap().block_type;
    match last_block_type {
        NumericBlockType::Space => {
            panic!("len-2 element was a space")
        }
        NumericBlockType::Data(i) => {
            last_id = i;
        }
    }

    if last_id == 0 {
        panic!("last data is 0");
    }


    while last_id > 0 {
        // println!("this is last id: {}", last_id);
        find_space_for_data_id(blocks, last_id);
        // println!("last idx: {}, block: {:?}", last_idx, blocks[last_idx]);
        last_id -= 1;
    }
}

fn find_space_for_data_id(blocks: &mut Vec<Block>, _data_id: u32) {

    blocks.push(Block{
        block_type: NumericBlockType::Space,
        start: 0,
        length: 0,
    });

    // println!("incoming blocks vec is\n{:?}", blocks);

    // filter out all blocks that are length 0, we do not need them.
    blocks.retain(|&b| b.length != 0);
    let mut length_needed = 0;
    let mut data_start = 0;

    // println!("incoming blocks filtered (retained):\n{:?}\n", blocks);
    // println!("starting the search for the next data block to replace. Incoming data ID is {}, so we need to move that, or a data block that has a lower ID", _data_id);

    // find the last data we have with id _data_id.
    let mut data_idx = blocks.len() - 1;
    // println!("starting the search at index {} in the new block", data_idx);
    while data_idx > 0 {
        // println!("-- while loop start at data idx {}", data_idx);
        match blocks[data_idx].block_type {
            NumericBlockType::Space => {
                // println!("-- X block was a space, decrementing idx and continuing");
                data_idx -= 1;
            }
            NumericBlockType::Data(id) => {
                if id != _data_id {
                    // println!("-- P block was data, but its id {} is too big, we're looking for {}. Decrementing and continuing.", id, _data_id);
                    data_idx -= 1;
                    continue;
                }

                // store the length needed for the space, and where this data starts, so we can
                // figure out whether a space is big enough, and whether it's to the left of it.
                length_needed = blocks[data_idx].length;
                data_start = blocks[data_idx].start;

                // println!("found the data block: {:?} at index {}, set length needed to {} and data start to {}",
                //          blocks[data_idx], data_idx, length_needed, data_start );
                break;
            }
        }
    }

    // println!("\nNext up let's find the space that's big enough to take our data");
    for (i, &block) in blocks.iter().enumerate() {
        // println!("-- i {}", i);
        match block.block_type {
            NumericBlockType::Space => {
                // println!("-- Ok: this is a space");
                if block.length < length_needed {
                    // println!("-- X: its length {} is too short for what we need {}", block.length, length_needed);
                    continue;
                }

                if block.start > data_start {
                    // println!("-- X: its start {} is too late for what we need {}, returning", block.start, data_start);
                    return;
                }

                // println!("-- Ok: found the block! {:?}", block);

                let replacement_space = vec![Block {
                    block_type: NumericBlockType::Space,
                    length: length_needed,
                    start: data_start,
                }];

                // println!("created a replacement space: {:?}", replacement_space);
                // println!("about to replace the data block we found with the space. The incoming blocks vec is\n{:?}", blocks);
                // replace the data from the end with an equal sized space
                let _replaced_block = blocks
                    .splice(data_idx..data_idx + 1, replacement_space)
                    .collect::<Vec<Block>>();

                // println!("replaced the data at idx with the replacement space\n{:?}", blocks);

                let moved_data = Block {
                    block_type: NumericBlockType::Data(_data_id),
                    start: block.start,
                    length: length_needed,
                };

                // println!("-- created a new data block for the same data: {:?}", moved_data);

                let diff = block.length - length_needed;
                let extra_space = Block {
                    block_type: NumericBlockType::Space,
                    length: diff,
                    start: block.start + length_needed
                };

                // println!("-- also created an extra space block so we can replace the space fully: {:?}", extra_space);
                // println!("-- for context, need to replace space length {} with data len {} + extra space len {}",
                // block.length, length_needed, diff);

                // println!("this is the vec we're doing a splice on\n{:?}", blocks);
                // println!("it has {} len and we're doing the range {}..{}", blocks.len(), i, i+1);

                let replacement = vec![moved_data, extra_space];
                
                let _replaced_space = blocks
                    .splice(i..i+1, replacement).collect::<Vec<Block>>();

                // println!("replacement supposedly done, whit is new_blocks?\n{:?}", blocks);
                return;
            }
            NumericBlockType::Data(_) => {
                // println!("-- X block was a data block, we're not messing with it");
                continue;
            }
        }
    }
}


fn bla( foo:&mut Vec<i32>) {
    foo.push(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_space_for_data_id() {
        let mut blocks = vec![
            Block{
                block_type: NumericBlockType::Data(0),
                start: 0,
                length: 2,
            },
            Block{
                block_type: NumericBlockType::Space,
                start: 2,
                length: 5,
            },
            Block{
                block_type: NumericBlockType::Data(1),
                start: 7,
                length: 3,
            },
        ];

        let want = vec![
            Block{
                block_type: NumericBlockType::Data(0),
                start: 0,
                length: 2,
            },
            Block{
                block_type: NumericBlockType::Data(1),
                start: 2,
                length: 3,
            },
            Block{
                block_type: NumericBlockType::Space,
                start: 5,
                length: 2,
            },
            Block{
                block_type: NumericBlockType::Space,
                start: 7,
                length: 3,
            },
        ];

        find_space_for_data_id(&mut blocks, 1);

        assert_eq!(want, blocks);
    }

    #[test]
    fn test_bla() {
        let mut input = vec![1, 2, 3, 4];
        bla(&mut input);

        assert_eq!(input, vec![1, 2, 3, 4, 0]);
    }
}