pub mod first;
pub mod fourth;
pub mod second;
pub mod third;
pub mod tree_node;
pub mod fifth;

pub mod linked_list;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {

    use tree_node::TreeNode;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn basics() {
        let mut list = first::List::new();

        list.push(12);
        list.push(13);
        list.push(14);

        assert_eq!(list.pop(), Some(14));
        assert_eq!(list.pop(), Some(13));
    }

    #[test]
    fn basics2() {
        let mut list = second::List::new();

        list.push(12);
        list.push(13);
        list.push(14);

        assert_eq!(list.pop(), Some(14));
        assert_eq!(list.pop(), Some(13));

        println!("{:?}", list.peek())
    }

    #[test]
    fn basics3() {
        let mut head = Box::new(TreeNode::new(1));
        let mut node1 = Box::new(TreeNode::new(2));
        let mut node2 = Box::new(TreeNode::new(3));
        let node3 = Box::new(TreeNode::new(4));

        node2.link(node3);
        node1.link(node2);
        head.link(node1);

        println!("{:?}", head)
    }

    #[test]
    fn into_iter() {
        let mut list = second::List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = second::List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = second::List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        while let Some(v) = iter.next() {
            println!("{}", v);
        }
    }

    #[test]
    fn basics_third() {
        let list = third::List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        println!("{:?}", list);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter_third() {
        let list = third::List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn fourth() {
        let mut list = fourth::List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        // ---- back -----

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn fourth_peek() {
        let mut list = fourth::List::new();

        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    }

    #[test]
    fn fourth_into_iter() {
        let mut list = fourth::List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }

    // #[test]
    // fn fifth_basic() {
    //     let mut list = fifth::List::new();
    //
    //     // Check empty list behaves right
    //     assert_eq!(list.pop(), None);
    //
    //     // Populate list
    //     list.push(1);
    //     list.push(2);
    //     list.push(3);
    //
    //     // Check normal removal
    //     assert_eq!(list.pop(), Some(1));
    //     assert_eq!(list.pop(), Some(2));
    //
    //     // Push some more just to make sure nothing's corrupted
    //     list.push(4);
    //     list.push(5);
    //
    //     // Check normal removal
    //     assert_eq!(list.pop(), Some(3));
    //     assert_eq!(list.pop(), Some(4));
    //
    //     // Check exhaustion
    //     assert_eq!(list.pop(), Some(5));
    //     assert_eq!(list.pop(), None);
    //
    //     // Check the exhaustion case fixed the pointer right
    //     list.push(6);
    //     list.push(7);
    //
    //     // Check normal removal
    //     assert_eq!(list.pop(), Some(6));
    //     assert_eq!(list.pop(), Some(7));
    //     assert_eq!(list.pop(), None);
    // }
}
