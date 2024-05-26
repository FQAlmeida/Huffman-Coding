use std::{collections::HashMap, iter::once};

fn frequency_counter(text: &String) -> HashMap<u8, usize> {
    text.as_bytes().iter().fold(HashMap::new(), |mut acc, &c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    })
}

fn frequency_list(frequency_counter: &HashMap<u8, usize>) -> Vec<(u8, usize)> {
    let mut frequency_list = frequency_counter
        .iter()
        .map(|(c, f)| (*c, *f))
        .collect::<Vec<_>>();
    frequency_list.sort_by(|(char_1, freq_1), (char_2, freq_2)| {
        freq_1.cmp(freq_2).reverse().then(char_1.cmp(char_2))
    });
    frequency_list
}

#[derive(Debug, PartialEq)]
struct HuffmanTreeNodeCharacter {
    character: u8,
    frequency: usize,
}

#[derive(Debug, PartialEq)]
struct HuffmanTreeNodeValue {
    value: usize,
    left: Option<Box<HuffmanTreeNode>>,
    right: Option<Box<HuffmanTreeNode>>,
}

#[derive(Debug, PartialEq)]
enum HuffmanTreeNode {
    Character(HuffmanTreeNodeCharacter),
    Value(HuffmanTreeNodeValue),
}

impl HuffmanTreeNode {
    fn value(&self) -> usize {
        match self {
            HuffmanTreeNode::Character(node) => node.frequency,
            HuffmanTreeNode::Value(node) => node.value,
        }
    }
}

fn huffman_tree(frequency_list: &[(u8, usize)]) -> HuffmanTreeNode {
    let character_node = HuffmanTreeNode::Character(HuffmanTreeNodeCharacter {
        character: frequency_list[0].0,
        frequency: frequency_list[0].1,
    });
    if frequency_list.len() == 1 {
        return character_node;
    }

    let value_node = huffman_tree(&frequency_list[1..]);

    let (left, right) = if character_node.value() >= value_node.value() {
        (character_node, value_node)
    } else {
        (value_node, character_node)
    };

    HuffmanTreeNode::Value(HuffmanTreeNodeValue {
        value: left.value() + right.value(),
        left: Some(Box::new(left)),
        right: Some(Box::new(right)),
    })
}

// TODO(Otavio): Change this to be a more memory efficient data structure
// like u8 -> (code: usize, length: u16)
// or u8 -> (code: usize, length: u8) if code can be bigger than a byte
type HuffmanCode = HashMap<u8, Vec<u8>>;

fn huffman_codes(tree: &HuffmanTreeNode) -> HuffmanCode {
    let mut codes = HuffmanCode::new();
    fn rec_huffman_codes(
        branch: &Option<Box<HuffmanTreeNode>>,
        code: &[u8],
        codes: &mut HuffmanCode,
    ) {
        match branch {
            Some(node_box) => match node_box.as_ref() {
                HuffmanTreeNode::Character(node) => {
                    codes.insert(node.character, code.to_vec());
                }
                HuffmanTreeNode::Value(node) => {
                    rec_huffman_codes(&node.left, &code.iter().chain(once(&b'0')).cloned().collect::<Vec<u8>>(), codes);
                    rec_huffman_codes(&node.right, &code.iter().chain(once(&b'1')).cloned().collect::<Vec<u8>>(), codes);
                }
            },
            None => {}
        }
    }
    match tree {
        HuffmanTreeNode::Character(node) => {
            codes.insert(node.character, vec![b'1']);
        }
        HuffmanTreeNode::Value(node) => {
            rec_huffman_codes(&node.left, &[b'0'].to_vec(), &mut codes);
            rec_huffman_codes(&node.right, &[b'1'].to_vec(), &mut codes);
        }
    }
    codes
}

fn huffman_encode_string(text: &String) -> (Vec<u8>, HashMap<Vec<u8>, u8>) {
    let frequency_counter = frequency_counter(text);
    let frequency_list = frequency_list(&frequency_counter);
    let tree = huffman_tree(&frequency_list);
    let codes = huffman_codes(&tree);
    let encoded = text
        .as_bytes()
        .iter()
        .flat_map(|c| {
            codes
                .get(c)
                .expect("Character not encoded")
                .iter()
                .map(|&c| if c == b'1' { 1 } else { 0 })
        })
        .collect();
    let decode_codes = codes
        .into_iter()
        .map(|(c, code)| (code, c))
        .collect::<HashMap<_, _>>();
    (encoded, decode_codes)
}

pub fn huffman_encode(text: &String) -> (Vec<u8>, HashMap<Vec<u8>, u8>) {
    let (encoded, codes) = huffman_encode_string(text);
    (
        encoded
            .chunks(8)
            .map(|bytes| bytes.iter().fold(0, |acc, b| acc << 1 | b))
            .collect::<Vec<u8>>(),
        codes,
    )
}

// This is how to decode properly:

// 1. Take a bit from the encoded string.
// 2. Start at the root of the tree.
// 3. Follow the edge denoted by that bit value to the adjacent node.
// 4. If the node is a leaf, then return the corresponding character.
// 5. Otherwise take the next bit from the encoded string and continue following down the tree until a leaf node is found.

// Example:

// 1. Start with the first bit in the encoded string, which is 1.
// 2. Start at the root of the tree.
// 3. The root is obviously not a leaf, so follow the edge marked 1 and we reach the leaf node, which is ''A'.
// 4. Then take the next bit, which is also 1. So, follow the step 3 above and return 'A' again.
// 5. Then take the next bit, which is 0 and start at the root again.
// 6. The edge 0 from the root leads to a non-leaf node (marked 4).
// 7. So, take the next bit from the encoded string, which is also 0 and continue following from the last node where we were at in the last step (which is the non-leaf node 4).
// 8. If we follow the edge 0, we reach a leaf node denoting character 'B'.
// 9. And so on..

// The important point is every time we find a character, we take the next bit from the encoded string and start at the root of the tree.

#[cfg(test)]
mod tests {
    // use rand::{distributions::Alphanumeric, rngs::StdRng, Rng, SeedableRng};

    use super::*;

    // #[test]
    // fn really_big_string_encode() {
    //     let r = StdRng::seed_from_u64(42);
    //     let text: String = r
    //         .sample_iter(&Alphanumeric)
    //         .take(1024 * 20)
    //         .map(char::from)
    //         .collect();
    //     let (encoded, _) = huffman_encode_string(&text);
    // }

    #[test]
    fn test_huffman_encode() {
        let text = String::from("AABCBAD");
        let (encoded, _) = huffman_encode(&text);
        let expected_encoded = vec![0b11000100, 0b00001011];
        assert_eq!(encoded, expected_encoded);
    }

    #[test]
    fn test_huffman_encode_string() {
        let text = String::from("AABCBAD");
        let (encoded, decode_codes) = huffman_encode_string(&text);
        let expected_encoded = vec![1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1];
        let expected_decode_codes = [
            (vec![b'1'], b'A'),
            (vec![b'0', b'0'], b'B'),
            (vec![b'0', b'1', b'0'], b'C'),
            (vec![b'0', b'1', b'1'], b'D'),
        ]
        .iter()
        .cloned()
        .collect();
        assert_eq!(encoded, expected_encoded);
        assert_eq!(decode_codes, expected_decode_codes);
    }

    #[test]
    fn test_huffman_codes() {
        let frequency_list = [(b'A', 3), (b'B', 2), (b'C', 1), (b'D', 1)];
        let tree = huffman_tree(&frequency_list);
        let result = huffman_codes(&tree);
        let expected = [
            (b'A', vec![b'1']),
            (b'B', vec![b'0', b'0']),
            (b'C', vec![b'0', b'1', b'0']),
            (b'D', vec![b'0', b'1', b'1']),
        ]
        .iter()
        .cloned()
        .collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_huffman_tree() {
        let frequency_list = [(b'A', 3), (b'B', 2), (b'C', 1), (b'D', 1)];
        let result = huffman_tree(&frequency_list);
        let expected = HuffmanTreeNode::Value(HuffmanTreeNodeValue {
            value: 7,
            left: Some(Box::new(HuffmanTreeNode::Value(HuffmanTreeNodeValue {
                value: 4,
                left: Some(Box::new(HuffmanTreeNode::Character(
                    HuffmanTreeNodeCharacter {
                        character: b'B',
                        frequency: 2,
                    },
                ))),
                right: Some(Box::new(HuffmanTreeNode::Value(HuffmanTreeNodeValue {
                    value: 2,
                    left: Some(Box::new(HuffmanTreeNode::Character(
                        HuffmanTreeNodeCharacter {
                            character: b'C',
                            frequency: 1,
                        },
                    ))),
                    right: Some(Box::new(HuffmanTreeNode::Character(
                        HuffmanTreeNodeCharacter {
                            character: b'D',
                            frequency: 1,
                        },
                    ))),
                }))),
            }))),
            right: Some(Box::new(HuffmanTreeNode::Character(
                HuffmanTreeNodeCharacter {
                    character: b'A',
                    frequency: 3,
                },
            ))),
        });
        assert_eq!(result, expected);
    }

    #[test]
    fn test_frequency_list() {
        let frequency_counter: HashMap<u8, usize> = [(b'A', 3), (b'B', 2), (b'C', 1), (b'D', 1)]
            .into_iter()
            .collect();
        let result = frequency_list(&frequency_counter);
        let expected = vec![(b'A', 3), (b'B', 2), (b'C', 1), (b'D', 1)];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_frequency_counter() {
        let text = "AABCBAD".to_string();
        let result = frequency_counter(&text);
        let expected: HashMap<u8, usize> = [(b'A', 3), (b'B', 2), (b'C', 1), (b'D', 1)]
            .into_iter()
            .collect();
        assert_eq!(result, expected);
    }
}
