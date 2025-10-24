#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head1 = list1;
    let mut head2 = list2;

    let mut output = Vec::new();

    loop {
        match (&head1, &head2) {
            (Some(list1), Some(list2)) => {
                if list1.val <= list2.val {
                    output.push(list1.val);

                    head1 = head1.unwrap().next;
                } else {
                    output.push(list2.val);

                    head2 = head2.unwrap().next;
                }
            }
            (Some(list1), None) => {
                output.push(list1.val);
                head1 = head1.unwrap().next;
            }
            (None, Some(list2)) => {
                output.push(list2.val);
                head2 = head2.unwrap().next;
            }
            _ => break,
        }
    }

    let mut output_linked_list = None;
    for i in output.into_iter().rev() {
        output_linked_list = match output_linked_list {
            None => Some(Box::new(ListNode::new(i))),
            Some(list) => Some(Box::new(ListNode {
                val: i,
                next: Some(list),
            })),
        };
    }

    output_linked_list
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn merge_lists() {
    let list1 = ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    };

    let list2 = ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    };

    let expected = ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 4, next: None })),
                    })),
                })),
            })),
        })),
    };

    assert_eq!(
        merge_two_lists(Some(Box::new(list1)), Some(Box::new(list2))),
        Some(Box::new(expected))
    );
}
