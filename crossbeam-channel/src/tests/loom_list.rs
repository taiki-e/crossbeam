use crate::*;
use loom::sync::atomic::*;
use loom::sync::Arc;
use loom::thread;

// #[test]
// fn spsc() {
//     const COUNT: usize = 4;
//     const THREADS: usize = 2;

//     loom::model(|| {
//         let (s, r) = unbounded();

//         let th1 = thread::spawn(move || {
//             for i in 0..COUNT {
//                 assert_eq!(r.recv(), Ok(i));
//             }
//             assert_eq!(r.recv(), Err(RecvError));
//         });
//         let th2 = thread::spawn(move || {
//             for i in 0..COUNT {
//                 s.send(i).unwrap();
//             }
//         });
//         th1.join().unwrap();
//         th2.join().unwrap();
//     });
// }

// #[test]
// fn basic_sequential_usage() {
//     loom::model(|| {
//         let (s, r) = unbounded();
//         s.send(5).unwrap();
//         let val = r.recv().unwrap();
//         assert_eq!(val, 5);
//     });
// }

#[test]
fn basic_parallel_usage() {
    loom::model(|| {
        let (s, r) = unbounded();
        thread::spawn(move || {
            s.send(5).unwrap();
        });
        let val = r.recv().unwrap();
        assert_eq!(val, 5);
    });
}

#[test]
fn commutative_senders() {
    loom::model(|| {
        let (s, r) = unbounded();
        let s2 = s.clone();
        thread::spawn(move || {
            s.send(5).unwrap();
        });
        thread::spawn(move || {
            s2.send(6).unwrap();
        });
        let mut val = r.recv().unwrap();
        val += r.recv().unwrap();
        assert_eq!(val, 11);
    });
}

fn ignore_result<A, B>(_: Result<A, B>) {}

#[test]
#[should_panic(expected = "assertion failed")]
fn non_commutative_senders1() {
    loom::model(|| {
        let (s, r) = unbounded();
        let s2 = s.clone();
        thread::spawn(move || {
            ignore_result(s.send(5));
        });
        thread::spawn(move || {
            ignore_result(s2.send(6));
        });
        let val = r.recv().unwrap();
        assert_eq!(val, 5);
        ignore_result(r.recv());
    });
}

#[test]
#[should_panic(expected = "assertion failed")]
fn non_commutative_senders2() {
    loom::model(|| {
        let (s, r) = unbounded();
        let s2 = s.clone();
        thread::spawn(move || {
            ignore_result(s.send(5));
        });
        thread::spawn(move || {
            ignore_result(s2.send(6));
        });
        let val = r.recv().unwrap();
        assert_eq!(val, 6);
        ignore_result(r.recv());
    });
}

// #[test]
// fn drop_receiver() {
//     loom::model(|| {
//         let (s, r) = unbounded();
//         s.send(1).unwrap();
//         s.send(2).unwrap();
//         assert_eq!(r.recv().unwrap(), 1);
//         assert_eq!(r.recv().unwrap(), 2);
//     });
// }
