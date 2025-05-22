//! # amt
//!
//! This is an implementation of an array-mapped trie.
//!
//!
//! ## Bitmaps
//!
//! The AMT bitmaps are structured like this, assume we're using a `u8` for the bitmap, with the
//! left 4 bits used for indexing, and the right 4 bits used for the offsets:
//!
//! ```text
//!   +-------+-------+
//!   |Bitmap |Offset |
//!   +-------+-------+
//! 1 | | | | |       |
//!   +-+-+-+-+-+-+-+-+
//! 2 | | | | |       |
//!   +-+-+-+-+-+-+-+-+
//! 3 | | | | |       |
//!   +-+-+-+-+-+-+-+-+
//! 4 | | | | |       |
//!   +-+-+-+-+-+-+-+-+
//! ```
//!
//! The AMT is described in Bagwell's paper using the following C++ code:
//! ```cpp
//! class AMTNode {
//!     unsigned int NodeCnt: 16, Chr: 16;
//!     #ifdef AMTSMALL
//!     union {
//!         AMTNode *IndexBaseA;
//!         int Value;
//!     };
//!     #else
//!     AMTNode *IndexBaseA;
//!     int Value;
//!     #endif
//!     AMTNode *IndexBase() {
//!         return IndexBaseA;
//!     }
//!
//!     void SIndexBase(AMTNode *IB) {
//!         IndexBaseA = IB;
//!     }
//!
//!     friend class CAMT;
//! }
//! ```

#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]


/// The bitmap implementation
#[derive(Debug, Clone, Default)]
struct AMTBitmap {
    // TODO: does the uint size matter here?
    /// The inner bitmap for elements
    map: u64,
}

impl AMTBitmap {
    /// Count the bits in the bitmap
    fn count_bits(&self) -> u32 {
        self.map.count_ones()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a default AMTNode for testing
    fn default_amt_node() -> AMTNode {
        AMTNode {
            node_count: 0,
            chr: 0,
            node_or_value: AMTNodeBase::Value(0), // Or some other default
        }
    }

    #[test]
    fn test_insert_stub() {
        let mut node = default_amt_node();
        node.insert(1, 100); // Call insert, no assertion yet
    }

    #[test]
    fn test_delete_stub() {
        let mut node = default_amt_node();
        node.delete(1); // Call delete, no assertion yet
    }

    #[test]
    fn test_search_stub() {
        let node = default_amt_node();
        node.search(1); // Call search, no assertion yet
    }
}

/// An enum used for the AMT node index base and value.
///
/// NOTE: AMTSmall in the paper / above code, uses a union to limit memory used. This could be
/// accomplished by using a rust union, but rust unions have a lot more restrictions / quirks /
/// nuances that make this optimization nontrivial to implement. This could be done in the future
/// but is left out for now.
#[derive(Debug, Clone)]
pub enum AMTNodeBase {
    // TODO: experiment with bumpalo here
    /// The base for the AMT node
    Base(Box<AMTNode>),
    /// The node has a value, and it is stored here
    Value(u32),
}

impl AMTNodeBase {
    /// Returns a reference to the current index base, if it exists.
    pub fn index_base(&self) -> Option<&Box<AMTNode>> {
        match self {
            AMTNodeBase::Base(base) => Some(base),
            AMTNodeBase::Value(_) => None,
        }
    }

    /// Returns the current value, if it exists.
    pub fn value(&self) -> Option<u32> {
        match self {
            AMTNodeBase::Base(_) => None,
            AMTNodeBase::Value(value) => Some(*value),
        }
    }
}

/// The AMT Node
#[derive(Debug, Clone)]
pub struct AMTNode {
    /// The number of nodes in the AMT
    node_count: u16,
    /// The character for the AMT node
    chr: u16,
    /// The index base for the AMT node
    node_or_value: AMTNodeBase,
}

impl AMTNode {
    /// Returns a reference to the current index base, if it exists.
    pub fn index_base(&self) -> Option<&Box<AMTNode>> {
        self.node_or_value.index_base()
    }

    /// Insert a value into the AMT
    pub fn insert(&mut self, _key: u32, _value: u32) -> Option<u32> {
        // TODO: implement
        None
    }

    /// Delete a value from the AMT
    pub fn delete(&mut self, _key: u32) -> Option<u32> {
        // TODO: implement
        None
    }

    /// Search for a value in the AMT
    pub fn search(&self, _key: u32) -> Option<u32> {
        // TODO: implement
        None
    }
}
