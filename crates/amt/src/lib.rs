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
    /// Creates a new, empty AMTBitmap.
    ///
    /// Initializes the inner `map` to 0.
    pub fn new() -> Self {
        Self { map: 0 }
    }

    /// Count the bits in the bitmap
    fn count_bits(&self) -> u32 {
        self.map.count_ones()
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
    /// Creates a new, empty AMTNode.
    ///
    /// Initializes `node_count` to 0, `chr` to 0, and `node_or_value`
    /// to `AMTNodeBase::Value(0)` as a starting default.
    pub fn new() -> Self {
        Self {
            node_count: 0,
            chr: 0, // Default character, can be refined later
            node_or_value: AMTNodeBase::Value(0), // Default initial state
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a default AMTNode for testing
    fn default_amt_node() -> AMTNode {
        AMTNode::new() // Utilize the new() constructor
    }

    #[test]
    fn test_insert_stub() {
        let mut node = default_amt_node();
        // Test with different keys and values
        assert_eq!(node.insert(1, 100), None);
        assert_eq!(node.insert(2, 200), None);
        assert_eq!(node.insert(0xFFFFFFFF, 300), None); // Max u32 key
        assert_eq!(node.insert(0, 0), None); // Zero key and value
    }

    #[test]
    fn test_delete_stub() {
        let mut node = default_amt_node();
        // Test with different keys
        assert_eq!(node.delete(1), None);
        assert_eq!(node.delete(0xFFFFFFFF), None);
        assert_eq!(node.delete(0), None);
    }

    #[test]
    fn test_search_stub() {
        let node = default_amt_node();
        // Test with different keys, expecting None as it's a stub
        assert_eq!(node.search(1), None);
        assert_eq!(node.search(0xFFFFFFFF), None);
        assert_eq!(node.search(0), None);
    }

    #[test]
    fn test_new_amt_node() {
        let node = AMTNode::new();
        assert_eq!(node.node_count, 0);
        assert_eq!(node.chr, 0);
        match node.node_or_value {
            AMTNodeBase::Value(val) => assert_eq!(val, 0),
            _ => panic!("Expected AMTNodeBase::Value(0) for a new node"),
        }
        assert!(node.index_base().is_none()); // Should not have a base node initially
    }

    #[test]
    fn test_amt_bitmap_count_bits_empty() {
        let bitmap = AMTBitmap { map: 0 };
        assert_eq!(bitmap.count_bits(), 0);
    }

    #[test]
    fn test_amt_bitmap_count_bits_full() {
        let bitmap = AMTBitmap { map: u64::MAX };
        assert_eq!(bitmap.count_bits(), 64);
    }

    #[test]
    fn test_amt_bitmap_count_bits_single_bit() {
        let bitmap1 = AMTBitmap { map: 1 };
        assert_eq!(bitmap1.count_bits(), 1);

        let bitmap2 = AMTBitmap { map: 1 << 5 };
        assert_eq!(bitmap2.count_bits(), 1);

        let bitmap3 = AMTBitmap { map: 1 << 63 };
        assert_eq!(bitmap3.count_bits(), 1);
    }

    #[test]
    fn test_amt_bitmap_count_bits_sparse() {
        let bitmap = AMTBitmap { map: 0b10101 }; // 3 bits set
        assert_eq!(bitmap.count_bits(), 3);
    }

    #[test]
    fn test_amt_bitmap_count_bits_dense() {
        let bitmap = AMTBitmap { map: 0b111000 }; // 3 bits set
        assert_eq!(bitmap.count_bits(), 3);

        let bitmap2 = AMTBitmap { map: 0xFF00FF00FF00FF00 }; // 32 bits set
        assert_eq!(bitmap2.count_bits(), 32);
    }

    #[test]
    fn test_new_amt_bitmap() {
        let bitmap = AMTBitmap::new();
        assert_eq!(bitmap.map, 0);
        assert_eq!(bitmap.count_bits(), 0);
    }
}
