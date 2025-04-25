fn main() {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut list1 = Vec::with_capacity(101);
    let mut list2 = Vec::with_capacity(101);

    let mut node = l1.unwrap();
    loop {
        list1.push(node.val);
        if let Some(next) = node.next {
            node = next;
        } else {
            break;
        }
    }

    let mut node = l2.unwrap();
    loop {
        list2.push(node.val);
        if let Some(next) = node.next {
            node = next;
        } else {
            break;
        }
    }

    if list1.len() < list2.len() {
        std::mem::swap(&mut list1, &mut list2);
    }

    let mut result_node = None;
    let mut carriage = 0;

    for i in 0..list1.len() {
        let item1 = list1.get(i).unwrap_or(&0);
        let item2 = list2.get(i).unwrap_or(&0);

        let result = item1 + item2 + carriage;

        carriage = result / 10;
        let final_result = result % 10;

        list1[i] = final_result
    }

    if carriage != 0 {
        list1.push(carriage);
    }

    for result in list1.into_iter().rev() {
        let node = ListNode {
            val: result,
            next: result_node,
        };

        result_node = Some(Box::new(node));
    }

    result_node
}
