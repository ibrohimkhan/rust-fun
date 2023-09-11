fn main() {
    let items = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let filter = FilterCondition::<u32> {
        item: 5,
        op: |a, b| a < b,
    };

    let result = custom_filter::<u32>(items, &filter);
    println!("{:?}", result);
}

struct FilterCondition<T: Copy> {
    item: T,
    op: fn(a: T, b: T) -> bool,
}

impl<T: Copy> FilterCondition<T> {
    pub fn is_match(&self, item: T) -> bool {
        (self.op)(self.item, item)
    }
}

fn custom_filter<T: Copy>(items: Vec<T>, my_filter: &FilterCondition<T>) -> Vec<T> {
    items.into_iter().filter(|&item| my_filter.is_match(item)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_greater_than_works() {
        let items = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let filter = FilterCondition::<u32> {
            item: 5,
            op: |a, b| a < b,
        };

        let result = custom_filter::<u32>(items, &filter);
        assert_eq!(result, vec![6, 7, 8, 9, 10]);
    }

    #[test]
    fn filter_less_than_works() {
        let items = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let filter = FilterCondition::<i32> {
            item: 5,
            op: |a, b| a > b,
        };

        let result = custom_filter::<i32>(items, &filter);
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn matching_some_value_works() {
        let items = vec![0, 1, 2, 3, 4, 3, 2, 7, 3, 9, 1];
        let filter = FilterCondition::<i32> {
            item: 3,
            op: |a, b| a == b,
        };

        let result = custom_filter::<i32>(items, &filter);
        assert_eq!(result, vec![3, 3, 3]);
    }
}
