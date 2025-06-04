pub struct Matcher<T> {
    matcher: Box<dyn Fn(T) -> bool>,
    subs: String,
}

pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T> Matcher<T> {
    pub fn new<F>(func: F, message: &str) -> Self
    where
        F: Fn(T) -> bool + 'static,
    {
        Matcher {
            matcher: Box::new(func),
            subs: message.to_string(),
        }
    }
}

impl<T: Clone> Fizzy<T> {
    pub fn new() -> Self {
        Fizzy {
            matchers: Vec::new(),
        }
    }

    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    pub fn apply<I>(self, _iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
        T: ToString + Clone,
    {
        _iter.map(move |item| {
            let item = item;
            let mut result = String::new();
            let mut has_sub = false;

            for matcher in &self.matchers {
                if (matcher.matcher)(item.clone()) {
                    result.push_str(&matcher.subs);
                    has_sub = true
                }
            }

            if has_sub { result } else { item.to_string() }
        })
    }
}

pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Clone + std::ops::Rem<Output = T> + PartialEq + std::fmt::Display + From<u8>,
{
    Fizzy::new()
        .add_matcher(Matcher {
            matcher: Box::new(|item: T| item % T::from(3) == T::from(0)),
            subs: "fizz".to_string(),
        })
        .add_matcher(Matcher {
            matcher: Box::new(|item: T| item % T::from(5) == T::from(0)),
            subs: "buzz".to_string(),
        })
}
