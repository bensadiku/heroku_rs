#[macro_export]

macro_rules! heroku_env {
    ($($element: expr => $value: expr),* $(,)?) => {{
        const SIZE: usize = ($crate::count![@COUNT; $($element),*]);
        #[allow(unused_mut)]
        let mut vs = ::std::collections::HashMap::with_capacity(SIZE);
        $(vs.insert($element, $value);)*
        vs
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($element:expr),*)=>{
        <[()]>::len(&[$($crate::count![@SUBST; $element]),*])
    };

    (@SUBST; $_element:expr)=>{ () };
}

#[test]
fn empty_map() {
    use std::collections::HashMap;
    let x: HashMap<&str, &str> = heroku_env![];
    assert_eq!(true, x.is_empty())
}

#[test]
fn trailing_comma() {
    let _x = heroku_env!["a" => "b",];
}

#[test]
fn one_element() {
    let x = heroku_env!["foo" => "baz"];
    assert_eq!(x.get("foo").unwrap(), &"baz");
}

#[test]
fn multiple_elements() {
    let x = heroku_env!["foo" => "baz", "a" => "b"];
    assert_eq!(x.get("foo").unwrap(), &"baz");
    assert_eq!(x.get("a").unwrap(), &"b");
    assert_eq!(x.get("test3"), None);
}

#[allow(dead_code)]
/// ```compile_fail
/// use std::collections::HashMap;
/// let x: HashMap<&str, &str> = heroku_env!["foo"];
/// ```
struct CompileFailTest;
