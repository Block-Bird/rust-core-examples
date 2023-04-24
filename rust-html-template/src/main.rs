macro_rules! html {
    ($tag:ident, { $($body:tt)* }) => {
        {
            let mut attrs = String::new();
            let mut children = String::new();
            html!(@body $($body)* => children, attrs);
            format!("<{tag}{attrs}>{children}</{tag}>", tag = stringify!($tag), attrs = attrs, children = children)
        }
    };
    ($tag:ident, { $($body:tt)* }, $($attr:ident = $value:expr),*) => {
        {
            let mut attrs = String::new();
            let mut children = String::new();
            html!(@body $($body)* => children, attrs);
            html!(@attr $($attr)* => attrs);
            format!("<{tag}{attrs}>{children}</{tag}>", tag = stringify!($tag), attrs = attrs, children = children)
        }
    };
    (@body $body:tt => $children:ident, $attrs:ident) => {
        {
            let child = html!($body);
            $children.push_str(&child);
        }
    };
    (@body $body:tt) => {
        html!($body)
    };
    (@attr) => {};
    (@attr $attr:ident = $value:expr => $attrs:ident) => {
        {
            $attrs.push_str(&format!(" {}=\"{}\"", stringify!($attr), $value));
            html!(@attr)
        }
    };
    (@attr $attr:ident = $value:expr, $($rest:tt)*) => {
        {
            html!(@attr $attr = $value => attrs);
            html!(@attr $($rest)*);
        }
    };
}

fn main() {
    let template = html!(div, {
        html!(h1, { "Hello, world!" })
    }, class = "container", id = "main");
    println!("{}", template);
}
