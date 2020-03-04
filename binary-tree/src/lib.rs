#![allow(dead_code)]

extern crate rand;

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

#[test]
fn binary_tree_size() {
    use std::mem::size_of;

    let word = size_of::<usize>();
    println!("word size: {}", word);
    assert_eq!(size_of::<BinaryTree<String>>(), word);

    type Triple = (
        &'static str,
        BinaryTree<&'static str>,
        BinaryTree<&'static str>,
    );
    assert_eq!(size_of::<Triple>(), 4 * word);
}

#[test]
fn test_hand_building_tree_of_planets() {
    use self::BinaryTree::*;

    let jupiter_tree = NonEmpty(Box::new(TreeNode {
        element: "Jupiter",
        left: Empty,
        right: Empty,
    }));
    let mercury_tree = NonEmpty(Box::new(TreeNode {
        element: "Mercury",
        left: Empty,
        right: Empty,
    }));
    let mars_tree = NonEmpty(Box::new(TreeNode {
        element: "Mars",
        left: jupiter_tree,
        right: mercury_tree,
    }));
    let venus_tree = NonEmpty(Box::new(TreeNode {
        element: "Venus",
        left: Empty,
        right: Empty,
    }));
    let uranus_tree = NonEmpty(Box::new(TreeNode {
        element: "Uranus",
        left: Empty,
        right: venus_tree,
    }));
    let tree = NonEmpty(Box::new(TreeNode {
        element: "Saturn",
        left: mars_tree,
        right: uranus_tree,
    }));

    assert_eq!(
        tree.walk(),
        vec!["Jupiter", "Mars", "Mercury", "Saturn", "Uranus", "Venus"]
    );
}

impl<T: Clone> BinaryTree<T> {
    fn walk(&self) -> Vec<T> {
        match self {
            BinaryTree::Empty => vec![],
            BinaryTree::NonEmpty(ref boxed) => {
                let mut result = boxed.left.walk();
                result.push(boxed.element.clone());
                result.extend(boxed.right.walk());
                result
            }
        }
    }
}

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }
}

#[test]
fn test_add_method_1() {
    let planets = vec!["Mercury", "Venus", "Mars", "Jupiter", "Saturn", "Uranus"];
    let mut tree = BinaryTree::Empty;
    for planet in planets {
        tree.add(planet)
    }

    assert_eq!(
        tree.walk(),
        vec!["Jupiter", "Mars", "Mercury", "Saturn", "Uranus", "Venus"]
    )
}

#[test]
fn test_add_method_2() {
    let mut tree = BinaryTree::Empty;
    tree.add("Mercury");
    tree.add("Venus");
    for planet in vec!["Mars", "Jupiter", "Saturn", "Uranus"] {
        tree.add(planet)
    }

    assert_eq!(
        tree.walk(),
        vec!["Jupiter", "Mars", "Mercury", "Saturn", "Uranus", "Venus"]
    )
}

use self::BinaryTree::*;

struct TreeIter<'a, T: 'a> {
    unvisited: Vec<&'a TreeNode<T>>,
}

impl<'a, T: 'a> TreeIter<'a, T> {
    fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
        while let NonEmpty(ref node) = *tree {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}

impl<T> BinaryTree<T> {
    fn iter(&self) -> TreeIter<T> {
        let mut iter = TreeIter {
            unvisited: Vec::new(),
        };
        iter.push_left_edge(self);
        iter
    }
}

impl<'a, T: 'a> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = TreeIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for TreeIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        let node = match self.unvisited.pop() {
            None => return None,
            Some(n) => n,
        };

        self.push_left_edge(&node.right);

        Some(&node.element)
    }
}

#[test]
fn external_iterator() {
    fn make_node<T>(left: BinaryTree<T>, element: T, right: BinaryTree<T>) -> BinaryTree<T> {
        NonEmpty(Box::new(TreeNode {
            left,
            element,
            right,
        }))
    }

    let subtree_l = make_node(Empty, "mecha", Empty);
    let subtree_rl = make_node(Empty, "droid", Empty);
    let subtree_r = make_node(subtree_rl, "robot", Empty);
    let tree = make_node(subtree_l, "Jaeger", subtree_r);

    let mut v = Vec::new();
    for kind in &tree {
        v.push(*kind);
    }
    assert_eq!(v, ["mecha", "Jaeger", "droid", "robot"]);

    let left_subtree = make_node(Empty, "mecha", Empty);
    let right_subtree = make_node(make_node(Empty, "droid", Empty), "robot", Empty);
    let tree = make_node(left_subtree, "Jaeger", right_subtree);

    let mut v = Vec::new();
    let mut iter = TreeIter { unvisited: vec![] };
    iter.push_left_edge(&tree);
    for kind in iter {
        v.push(*kind);
    }
    assert_eq!(v, ["mecha", "Jaeger", "droid", "robot"]);

    let mut v = Vec::new();
    for kind in &tree {
        v.push(*kind);
    }
    assert_eq!(v, ["mecha", "Jaeger", "droid", "robot"]);

    let mut v = Vec::new();
    let mut state = tree.into_iter();
    while let Some(kind) = state.next() {
        v.push(*kind);
    }
    assert_eq!(v, ["mecha", "Jaeger", "droid", "robot"]);

    assert_eq!(
        tree.iter()
            .map(|name| format!("mega-{}", name))
            .collect::<Vec<_>>(),
        vec!["mega-mecha", "mega-Jaeger", "mega-droid", "mega-robot"]
    );

    let mut iterator = tree.into_iter();
    assert_eq!(iterator.next(), Some(&"mecha"));
    assert_eq!(iterator.next(), Some(&"Jaeger"));
    assert_eq!(iterator.next(), Some(&"droid"));
    assert_eq!(iterator.next(), Some(&"robot"));
    assert_eq!(iterator.next(), None);
}
