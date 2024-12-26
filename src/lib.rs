pub mod first;
pub mod tree_node;
pub mod second;
pub mod third;

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
        let mut node3 = Box::new(TreeNode::new(4));

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
}
