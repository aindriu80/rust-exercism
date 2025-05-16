/// Yields each item of a and then each item of b
pub fn append<I, J>(a: I, b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    // Create our own chaining iterator instead of using the built-in chain()
    let mut a_iter = a;
    let mut b_iter = b;
    let mut exhausted_a = false;

    std::iter::from_fn(move || {
        if !exhausted_a {
            // Try to get the next item from the first iterator
            match a_iter.next() {
                Some(item) => Some(item),
                None => {
                    // First iterator is exhausted, switch to the second
                    exhausted_a = true;
                    b_iter.next()
                }
            }
        } else {
            // We've already exhausted the first iterator, just yield from the second
            b_iter.next()
        }
    })
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(iters: impl Iterator<Item = I>) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    // I::Item: Iterator,
{
    // We'll collect all iterators into a vector first
    let mut iterators: Vec<I> = iters.collect();

    // Keep track of which iterator we're currently processing
    let mut current_iter_index = 0;

    std::iter::from_fn(move || {
        // Loop until we find an iterator with a next value or exhaust all iterators
        while current_iter_index < iterators.len() {
            // Try to get the next value from the current iterator
            if let Some(value) = iterators[current_iter_index].next() {
                return Some(value);
            }

            // Current iterator is exhausted, move to the next one
            current_iter_index += 1;
        }

        // All iterators are exhausted
        None
    })
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    let mut iterator = iter;

    std::iter::from_fn(move || {
        // Keep requesting the next item until we find one that passes the predicate
        // or until the iterator is exhausted
        loop {
            match iterator.next() {
                // Found an item, check if it passes the predicate
                Some(item) => {
                    if predicate(&item) {
                        // Item passes the predicate, yield it
                        return Some(item);
                    }
                    // Item doesn't pass the predicate, continue to the next one
                }
                // Iterator is exhausted
                None => return None,
            }
        }
    })
}
/// Returns the total number of items within iter
pub fn length<I: Iterator>(iter: I) -> usize {
    let mut count = 0;
    let mut iterator = iter;

    while let Some(_) = iterator.next() {
        count += 1;
    }

    count
}
/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    let mut iterator = iter;

    std::iter::from_fn(move || match iterator.next() {
        Some(item) => Some(function(item)),
        None => None,
    })
}

/// Starting with initial, fold (reduce) each iter item into the accumulator from the left
pub fn foldl<I, F, U>(iter: I, initial: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    let mut result = initial;
    let mut iterator = iter;

    while let Some(item) = iterator.next() {
        result = function(result, item);
    }

    result
}

/// Starting with initial, fold (reduce) each iter item into the accumulator from the right
pub fn foldr<I, F, U>(iter: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    let mut result = initial;
    let mut iterator = iter.rev(); // Reverse the iterator

    while let Some(item) = iterator.next() {
        result = function(result, item);
    }

    result
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(iter: I) -> impl Iterator<Item = I::Item> {
    let mut iterator = iter.rev();

    std::iter::from_fn(move || iterator.next())
}
