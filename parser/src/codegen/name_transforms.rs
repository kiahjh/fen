pub fn snake_to_camel(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize = false;

    for c in s.chars() {
        if c == '_' {
            capitalize = true;
        } else if capitalize {
            result.push(c.to_ascii_uppercase());
            capitalize = false;
        } else {
            result.push(c);
        }
    }

    result
}

pub fn snake_to_pascal(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize = true;

    for c in s.chars() {
        if c == '_' {
            capitalize = true;
        } else if capitalize {
            result.push(c.to_ascii_uppercase());
            capitalize = false;
        } else {
            result.push(c);
        }
    }

    result
}

pub fn pascal_to_camel(s: &str) -> String {
    let mut result = String::new();

    for (i, c) in s.chars().enumerate() {
        if i == 0 {
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }

    result
}

pub fn pascal_to_kebab(s: &str) -> String {
    let mut result = String::new();

    for (i, c) in s.chars().enumerate() {
        if i > 0 && c.is_ascii_uppercase() {
            result.push('-');
        }
        result.push(c.to_ascii_lowercase());
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_snake_to_camel() {
        assert_eq!(super::snake_to_camel("hello_world"), "helloWorld");
        assert_eq!(super::snake_to_camel("foo_bar_baz"), "fooBarBaz");
        assert_eq!(super::snake_to_camel("foo"), "foo");
        assert_eq!(super::snake_to_camel("foo_bar"), "fooBar");
        assert_eq!(super::snake_to_camel("foo_bar_baz_qux"), "fooBarBazQux");
    }

    #[test]
    fn test_snake_to_pascal() {
        assert_eq!(super::snake_to_pascal("hello_world"), "HelloWorld");
        assert_eq!(super::snake_to_pascal("foo_bar_baz"), "FooBarBaz");
        assert_eq!(super::snake_to_pascal("foo"), "Foo");
        assert_eq!(super::snake_to_pascal("foo_bar"), "FooBar");
        assert_eq!(super::snake_to_pascal("foo_bar_baz_qux"), "FooBarBazQux");
    }

    #[test]
    fn test_pascal_to_camel() {
        assert_eq!(super::pascal_to_camel("HelloWorld"), "helloWorld");
        assert_eq!(super::pascal_to_camel("FooBarBaz"), "fooBarBaz");
        assert_eq!(super::pascal_to_camel("Foo"), "foo");
        assert_eq!(super::pascal_to_camel("FooBar"), "fooBar");
        assert_eq!(super::pascal_to_camel("FooBarBazQux"), "fooBarBazQux");
    }

    #[test]
    fn test_pascal_to_kebab() {
        assert_eq!(super::pascal_to_kebab("HelloWorld"), "hello-world");
        assert_eq!(super::pascal_to_kebab("FooBarBaz"), "foo-bar-baz");
        assert_eq!(super::pascal_to_kebab("Foo"), "foo");
        assert_eq!(super::pascal_to_kebab("FooBar"), "foo-bar");
        assert_eq!(super::pascal_to_kebab("FooBarBazQux"), "foo-bar-baz-qux");
    }
}
