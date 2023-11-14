fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut result = Vec::new();

    for item in a {
        if !b.contains(&item) {
            result.push(item);
        }
    }

    result
}