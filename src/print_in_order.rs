/**
 * 1114. Print in Order
 */
use std::sync::{Condvar, Mutex};
type Job = Box<dyn Fn() + Send + 'static>;

trait Foo {
    fn first(&self, print_first: Job);
    fn second(&self, print_second: Job);
    fn third(&self, print_third: Job);
}

struct Solution {
    mx: Mutex<i32>,
    cvar: Condvar,
}
impl Solution {
    fn new() -> Self {
        Self {
            mx: Mutex::new(0),
            cvar: Condvar::new(),
        }
    }
}

impl Foo for Solution {
    fn first(&self, print_first: Job) {
        let mut processed = self.mx.lock().unwrap();
        print_first();
        *processed = 1;
        self.cvar.notify_all();
    }

    fn second(&self, print_second: Job) {
        let mut processed = self.mx.lock().unwrap();
        while *processed != 1 {
            processed = self.cvar.wait(processed).unwrap();
        }
        print_second();
        *processed = 2;
        self.cvar.notify_all();
    }

    fn third(&self, print_third: Job) {
        let mut processed = self.mx.lock().unwrap();
        while *processed != 2 {
            processed = self.cvar.wait(processed).unwrap();
        }
        print_third();
    }
}

#[cfg(test)]
mod tests {
    use std::marker::{Send, Sync};
    use std::sync::{mpsc, Arc};
    use std::thread;

    use super::{Foo, Solution};

    struct Invoker {
        foo: Box<dyn Foo + Send + Sync + 'static>,
    }

    impl Invoker {
        fn invoke(&self, sender: mpsc::Sender<String>, i: i32) {
            match i {
                1 => &self.foo.first(Box::new(move || {
                    let _ = sender.send("first".to_string());
                })),
                2 => &self.foo.second(Box::new(move || {
                    let _ = sender.send("second".to_string());
                })),
                3 => &self.foo.third(Box::new(move || {
                    let _ = sender.send("third".to_string());
                })),
                _ => panic!("wrong thread number"),
            };
        }
    }

    static NTHREADS: i32 = 3;

    fn assert_order(order: &[i32; 3]) {
        let invoker = Arc::new(Invoker {
            foo: Box::new(Solution::new()),
        });
        let (sender, receiver) = mpsc::channel::<String>();

        thread::scope(move |t| {
            t.spawn({
                let ls = sender.clone();
                let inv = invoker.clone();
                move || inv.invoke(ls, order[0])
            });
            t.spawn({
                let ls = sender.clone();
                let inv = invoker.clone();
                move || inv.invoke(ls, order[1])
            });
            t.spawn({
                let ls = sender.clone();
                let inv = invoker.clone();
                move || inv.invoke(ls, order[2])
            });
        });

        let mut res = Vec::with_capacity(NTHREADS as usize);
        for _ in 0..NTHREADS {
            // The `recv` method picks a message from the channel
            // `recv` will block the current thread if there are no messages available
            match receiver.recv() {
                Ok(v) => res.push(v),
                Err(e) => println!("{:?}", e),
            }
        }

        assert_eq!(res.join(""), "firstsecondthird".to_string());
    }

    #[test]
    fn test_case_1() {
        assert_order(&[1, 2, 3]);
    }

    #[test]
    fn test_case_2() {
        assert_order(&[1, 3, 2]);
    }

    #[test]
    fn test_case_3() {
        assert_order(&[2, 1, 3]);
    }

    #[test]
    fn test_case_4() {
        assert_order(&[2, 3, 1]);
    }

    #[test]
    fn test_case_5() {
        assert_order(&[3, 1, 2]);
    }

    #[test]
    fn test_case_6() {
        assert_order(&[3, 2, 1]);
    }

    // #[ignore]
    // #[test]
    // fn test_case_2() {
    //     let nums = [0, 1].to_vec();

    //     let res = Solution::permute(nums);

    //     assert_answer(res, &[&[0, 1], &[1, 0]]);
    // }
}
