// use std::{pin::Pin, task::Context, thread, time::Duration};

// use futures::task::Poll;
// use futures_core::Stream;

pub struct DataStream {
    data: Vec<u8>,
    polled: bool,
    wake_immediately: bool,
}

// impl Stream for DataStream {
//     type Item = u8;

//     fn poll_next(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Option<Self::Item>> {
//         if !self.polled {
//             if !self.wake_immediately {
//                 let waker = ctx.waker().clone();
//                 let sleep_time =
//                     Duration::from_millis(*self.data.first().unwrap_or(&0) as u64 / 10);
//                 thread::spawn(move || {
//                     thread::sleep(sleep_time);
//                     waker.wake_by_ref();
//                 });
//             } else {
//                 ctx.waker().wake_by_ref();
//             }
//             self.polled = true;
//             Poll::Pending
//         } else {
//             self.polled = false;
//             Poll::Ready(self.data.pop())
//         }
//     }
// }
