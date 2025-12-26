use std::hint::black_box;

#[derive(Clone, Debug)]
pub struct Node {
    pub value: i32,
    pub next: Option<Box<Node>>,
}


impl Node {
    pub fn new(val: i32) -> Self {
        Self {
            value: val,
            next: None,
        }
    }
}

#[inline(never)]
pub fn list_add(n: usize) -> Box<Node> {
    let mut head: Option<Box<Node>> = None;

    for i in 0..n {
        let mut node = Box::new(Node::new((i as i32) ^ 0x5a5a5a5a));
        node.next = head;
        head = Some(node);
        head = black_box(head);
    }

    head.expect("Failed to create add items to list")
}


#[inline(never)]
pub fn list_reverse(mut head: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut prev: Option<Box<Node>> = None;


    while let Some(mut curr) = head {
        head = curr.next.take();
        curr.next = prev;
        prev = Some(curr);
        prev = black_box(prev);
    }

    prev
}


#[inline(never)]
pub fn list_bubble_sort(mut head: Option<Box<Node>>) -> Option<Box<Node>> {
    if head.is_none() {
        return head;
    }

    let mut swapped = true;

    while swapped {
        swapped = false;
        let mut curr = head.as_mut();

        while let Some(node) = curr {
            if let Some(next) = node.next.as_mut() {
                if node.value > next.value {
                    std::mem::swap(&mut node.value, &mut next.value);
                    swapped = true;
                }
            }
            curr = node.next.as_mut();
            curr = black_box(curr);
        }
    }

    head
}

#[inline(never)]
pub fn list_checksum(head: &Option<Box<Node>>) -> u64 {
    let mut sum = 0u64;
    let mut curr = head.as_ref();

    while let Some(node) = curr {
        sum = sum.wrapping_add(node.value as u64);
        curr = node.next.as_ref();
        curr = black_box(curr);
    }

    sum
}
