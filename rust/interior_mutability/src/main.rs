pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: you are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota");
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn sends_message_at_75_percent() {
        let fake_messenger = FakeMessenger::new();
        let mut limit_tracker = LimitTracker::new(&fake_messenger, 100);

        limit_tracker.set_value(75);

        let len = fake_messenger.sent_messages.borrow().len();
        assert_eq!(len, 1)
    }

    struct FakeMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl FakeMessenger {
        fn new() -> FakeMessenger {
            FakeMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for FakeMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(msg.to_string());
        }
    }
}
