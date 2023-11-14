fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
    where
        T: IntoIterator,
        T::Item: PartialEq + Clone,
{
    let mut iter = sequence.into_iter();
    let mut result = Vec::new();

    if let Some(mut prev) = iter.next() {
        result.push(prev.clone());

        for item in iter {
            if item != prev {
                result.push(item.clone());
            }
            prev = item;
        }
    }

    result
}