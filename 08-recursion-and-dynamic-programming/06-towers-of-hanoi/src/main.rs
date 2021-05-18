type Stack = Vec<u64>;

/*
 *  n = 1 => D1 T1 -> T3
 *  n = 2 => D1 T1 -> T2; D2 T1 -> T3; D1 T2 -> T3
 *  n = 3 => D1 T1 -> T3; D2 T1 -> T2; D1 T3 ->
*/
fn towers_of_hanoi(n: u64) -> (Stack, Stack, Stack) {
    fn inner(n: u64, src: &mut Stack, dest: &mut Stack, buf: &mut Stack) {
        if n == 0 {
            return;
        }

        // move the top n - 1 disks from the origin to the buffer
        inner(n - 1, src, buf, dest);

        // move top from origin to destination
        if let Some(d) = src.pop() {
            dest.push(d);
        }

        // move top n - 1 disks from buffer to destination, using the origin as a buffer
        inner(n - 1, buf, dest, src);
    }

    let mut src = (0..n).into_iter().collect::<Stack>();
    let mut buf = Vec::with_capacity(n as usize);
    let mut dst = Vec::with_capacity(n as usize);

    inner(n, &mut src, &mut buf, &mut dst);

    (src, dst, buf)
}

fn main() {
    dbg!(towers_of_hanoi(5));
}
