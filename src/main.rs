use std::collections::HashMap;

struct Block {
    id: u16
}

const BLOCK_AIR: Block = Block { id: 0 };

impl Default for Block {
    fn default() -> Block { BLOCK_AIR }
}

const CHUNK_WIDTH: usize = 32;
const CHUNK_AREA: usize = 32 * 32;
const CHUNK_VOLUME: usize = 32 * 32 * 32;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct ChunkIndex {
    x: usize,
    y: usize,
    z: usize
}

impl ChunkIndex {
    fn to_array_index(&self) -> usize {
        self.x + self.y * CHUNK_WIDTH + self.z * CHUNK_AREA
    }
}

struct Chunk {
    blocks: [Block; CHUNK_VOLUME]
}

impl Chunk {
    fn get_block(&self, pos: &ChunkIndex) -> &Block {
        &self.blocks[pos.to_array_index()]
    }

    fn get_mut_block(&mut self, pos: &ChunkIndex) -> &Block {
        &mut self.blocks[pos.to_array_index()]
    }
    
    fn set_block(&mut self, pos: &ChunkIndex, block: Block) {
        self.blocks[pos.to_array_index()] = block;
    }
    
    fn remove_block(&mut self, pos: &ChunkIndex) {
        self.set_block(pos, Block::default())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64
}

// this is implemented natively in rust nightly
fn div_euc(x: i64, rhs: i64) -> i64 {
    let q = x / rhs;
    if x % rhs < 0 {
        return if rhs > 0 { q - 1 } else { q + 1 }
    }
    q
}

fn mod_euc(x: i64, rhs: i64) -> i64 {
    let r = x % rhs;
    if r < 0 {
        return if rhs > 0 { r + rhs } else { r - rhs }
    }
    r
}

fn get_single_chunk_axis(x: i64) -> i64 {
    div_euc(x, CHUNK_WIDTH as i64)
}

fn get_single_chunk_index(x: i64) -> usize {
    mod_euc(x, CHUNK_WIDTH as i64) as usize
}

impl Coordinate {
    fn get_chunk_coordinate(&self) -> Coordinate {
        Coordinate {
            x: get_single_chunk_axis(self.x),
            y: get_single_chunk_axis(self.y),
            z: get_single_chunk_axis(self.z)
        }
    }

    fn get_chunk_index(&self) -> ChunkIndex {
        ChunkIndex {
            x: get_single_chunk_index(self.x),
            y: get_single_chunk_index(self.y),
            z: get_single_chunk_index(self.z)
        }
    }
}
        
struct World {
    chunks: HashMap<Coordinate, Chunk>
}

impl World {
    fn get_chunk_containing(&self, pos: &Coordinate) -> &Chunk {
        self.chunks.get(&pos.get_chunk_coordinate()).expect("chunk loading not yet implemented")
    }

    fn get_mut_chunk_containing(&mut self, pos: &Coordinate) -> &mut Chunk {
        self.chunks.get_mut(&pos.get_chunk_coordinate()).expect("chunk loading not yet implemented")
    }
    
    fn get_block(&self, pos: &Coordinate) -> &Block {
        self.get_chunk_containing(pos).borrow_block(&pos.get_chunk_index())
    }

    fn get_borrow_mut_block(&mut self, pos: &Coordinate) -> &Block {
        self.get_mut_chunk_containing(pos).borrow_mut_block(&pos.get_chunk_index())
    }
    
    fn set_block(&mut self, pos: &Coordinate, block: Block) {
        self.get_mut_chunk_containing(pos).set_block(&pos.get_chunk_index(), block)
    }
    
    fn remove_block(&mut self, pos: &Coordinate) {
        self.get_mut_chunk_containing(pos).remove_block(&pos.get_chunk_index())
    }
}   

fn main() {
    
}
