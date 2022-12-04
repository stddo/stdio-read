use std::io::BufRead;
use std::io::stdin;

/// Reads stdin lines, parses them into specified types and assign values to variables.
/// You can specify either multiple variables with types that implement trait `FromStr`
/// or a `Vec` containing types that implement trait `FromStr`.
///
/// # Examples
///
/// ```
/// use stdio_read::read_vars;
///
/// read_vars!(x: u16, n: usize);
/// println!("x = {}, n = {}", x, n);
///
/// read_vars!(els: Vec<isize>);
/// for el in els {
///     println!("{}, ", el)
/// }
/// ```
#[macro_export]
macro_rules! read_vars {
    ($v:ident:Vec<$t:ty>) => {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let $v: Vec<$t> = buffer.trim().split(" ").map(|x| x.parse().unwrap()).collect::<Vec<_>>();
    };

    (@step $_idx:expr, @data $_data:ident,) => {};

    (@step $idx:expr, @data $data:ident, $v:ident:$t:ty, $($v_rest:ident:$t_rest:ty,)*) => {
        let $v:$t = $data[$idx].parse().unwrap();
        read_vars!(@step $idx + 1usize, @data $data, $($v_rest:$t_rest,)*);
    };

    ($($v:ident:$t:ty),*) => {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let data = buffer.trim().split(" ").collect::<Vec<_>>();
        read_vars!(@step 0usize, @data data, $($v:$t,)*);
    };
}