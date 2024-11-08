struct FilterCondition<T> {
    value: T,
}

// Add PartialEq bound to the impl block to allow equality comparison
impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        self.value == *item
    }
}

// Add PartialEq and Clone constraints to T and specify the return type
fn custom_filter<T: PartialEq + Clone>(collection: &Vec<T>, filter: FilterCondition<T>) -> Vec<T> {
    let mut result = Vec::new();
    for item in collection {
        if filter.is_match(item) {
            result.push(item.clone());
        }
    }
    result
}

fn main() {
    let new_collection = vec![2, 4, 8, 8, 6, 1, 3];

    let filter_num = FilterCondition { value: 2 };

    let filtered_nums = custom_filter(&new_collection, filter_num);
    println!("Filtered numbers: {:?}", filtered_nums); // Output: [2]
}
