#[path = "src/tables.rs"]
mod tables;

use std::cmp::Ordering;
use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use block::{Block, BlockSize};
use tables::{JoiningType, JOINING_TYPE};

fn main() {
    let output_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("joining_type.rs");
    let joining_type_table = CompiledTable::compile(JOINING_TYPE);

    write_joining_type_table(&output_path, &joining_type_table);
}

struct CompiledTable<T>
where
    T: Default + BlockSize + Copy,
{
    blocks: Vec<(u32, Block<T>)>,
    address_to_block_index: Vec<(u32, usize)>,
    last_code_point: u32,
}

impl<T> CompiledTable<T>
where
    T: Default + BlockSize + Copy + Eq,
{
    fn compile(table: &[(u32, u32, T)]) -> Self {
        let last_index = T::last_index();
        let shift = last_index.count_ones();
        let mut blocks = Vec::new();
        let mut address_to_block_index = Vec::new();

        let &(start, _, _) = table.iter().min_by_key(|(start, _, _)| start).unwrap();
        let &(_, end, _) = table.iter().max_by_key(|(_, end, _)| end).unwrap();
        let last_code_point = end;

        // Extend end to the end of the last block to ensure the last block is written out
        let end_block_address = end & (!last_index as u32);
        let end = end_block_address + T::SIZE as u32;

        let mut block = Block::new();
        for codepoint in start..=end {
            let joining_type = lookup(codepoint, table);
            let block_address = (codepoint >> shift).saturating_sub(1) << shift;

            // This is the first codepoint in this block, write out the previous block
            if codepoint != 0 && (codepoint & u32::try_from(last_index).unwrap()) == 0 {
                if let Some(index) = blocks.iter().position(|(_, candidate)| candidate == &block) {
                    address_to_block_index.push((block_address, index));
                } else {
                    // Add the block if it's new
                    address_to_block_index.push((block_address, blocks.len()));
                    blocks.push((block_address, block.clone()));
                }

                block.reset();
            }

            block[usize::try_from(codepoint).unwrap() & last_index] = joining_type;
        }

        CompiledTable {
            blocks,
            address_to_block_index,
            last_code_point,
        }
    }
}

fn write_joining_type_table(path: &Path, compiled_table: &CompiledTable<JoiningType>) {
    let mut output =
        File::create(&path).expect(&format!("unable to open {}", path.to_string_lossy()));

    writeln!(output, "use crate::JoiningType;").unwrap();
    writeln!(output, "use crate::JoiningType::*;").unwrap();

    writeln!(
        output,
        "\nconst LAST_CODEPOINT: u32 = 0x{:X};",
        compiled_table.last_code_point
    )
    .unwrap();
    writeln!(output, "\nconst BLOCK_SIZE: usize = {};", JoiningType::SIZE).unwrap();

    // Write out the blocks in address order
    writeln!(
        output,
        "\nconst JOINING_TYPE_BLOCKS: [JoiningType; {}] = [",
        compiled_table.blocks.len() * JoiningType::SIZE
    )
    .unwrap();

    for (address, block) in &compiled_table.blocks {
        writeln!(output, "// BLOCK: {:04X}\n", address).unwrap();
        for (i, joining_type) in block.iter().enumerate() {
            if i != 0 && (i & 0xF) == 0 {
                writeln!(output).unwrap();
            }

            write!(output, "{:?},", joining_type).unwrap();
        }

        write!(output, "\n\n").unwrap();
    }
    writeln!(output, "];").unwrap();

    write!(output, "\n\n").unwrap();

    // Write out constants for the block offsets
    for (index, (address, _)) in compiled_table.blocks.iter().enumerate() {
        writeln!(
            output,
            "const BLOCK_OFFSET_{:04X}: u16 = 0x{:04X};",
            address,
            index * JoiningType::SIZE
        )
        .unwrap();
    }

    // Write out the array that maps joining types to offsets
    writeln!(
        output,
        "\nconst JOINING_TYPE_BLOCK_OFFSETS: [u16; {}] = [",
        compiled_table.address_to_block_index.len()
    )
    .unwrap();
    for &(_, index) in &compiled_table.address_to_block_index {
        let (block_address, _) = compiled_table.blocks[index];
        writeln!(output, "    BLOCK_OFFSET_{:04X},", block_address).unwrap();
    }
    writeln!(output, "];").unwrap();
}

/// Lookup this code point in `table`
fn lookup<T>(codepoint: u32, table: &[(u32, u32, T)]) -> T
where
    T: Default + BlockSize + Eq + Copy,
{
    table
        .binary_search_by(|&(start, end, _)| {
            if codepoint < start {
                Ordering::Greater
            } else if codepoint > end {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        })
        .ok()
        .map(|idx| table[idx].2)
        .unwrap_or_default()
}

impl Default for JoiningType {
    fn default() -> Self {
        JoiningType::NonJoining
    }
}

impl BlockSize for JoiningType {
    const SIZE: usize = 256;
}

mod block {
    use std::ops::{Index, IndexMut};

    pub trait BlockSize {
        const SIZE: usize;

        fn last_index() -> usize {
            Self::SIZE - 1
        }
    }

    /// A fixed size block
    ///
    /// Ideally this would be an array but that doesn't work until const generics are stabilised
    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    pub struct Block<T>
    where
        T: Default + BlockSize + Copy,
    {
        data: Vec<T>,
    }

    impl<T> Block<T>
    where
        T: Default + BlockSize + Copy,
    {
        pub fn new() -> Self {
            Block {
                data: vec![T::default(); T::SIZE],
            }
        }

        pub fn reset(&mut self) {
            self.data.iter_mut().for_each(|val| *val = T::default());
        }

        pub fn iter(&self) -> impl Iterator<Item = &T> {
            self.data.iter()
        }
    }

    impl<T> Index<usize> for Block<T>
    where
        T: Default + BlockSize + Copy,
    {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl<T> IndexMut<usize> for Block<T>
    where
        T: Default + BlockSize + Copy,
    {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            self.data.index_mut(index)
        }
    }
}
