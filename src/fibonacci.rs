pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut array = vec![0; (n + 1) as usize];

    array[0] = 0;
    array[1] = 1;

    for i in 2..=n as usize {
        array[i] = array[i - 1] + array[i - 2];
    }

    array[n as usize]
}
