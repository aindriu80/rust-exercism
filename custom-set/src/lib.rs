#[derive(Debug)]
pub struct CustomSet<T> {
    data: Vec<T>,
}
impl<T: PartialEq> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.data.len() != other.data.len() {
            return false;
        }
        self.data.iter().all(|elem| other.data.contains(elem))
    }
}

impl<T: PartialEq> Eq for CustomSet<T> {}

impl<T> CustomSet<T>
where
    T: Clone + PartialEq,
{
    pub fn new(input: &[T]) -> Self {
        let mut data = Vec::new();
        for item in input {
            if !data.contains(item) {
                data.push(item.clone());
            }
        }
        CustomSet { data }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.data.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.data.push(element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        if self.is_empty() {
            return true;
        } else if other.is_empty() {
            return false;
        } else {
            self.data.iter().all(|elem| other.contains(elem))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        if self.is_empty() || other.is_empty() {
            return true;
        } else {
            self.data.iter().all(|elem| !other.contains(elem))
        }
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let data = self
            .data
            .iter()
            .filter(|elem| other.contains(elem))
            .cloned()
            .collect::<Vec<T>>();
        CustomSet { data }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let data = self
            .data
            .iter()
            .filter(|elem| !other.contains(elem))
            .cloned()
            .collect::<Vec<T>>();
        CustomSet { data }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut data = self.data.clone();
        for elem in &other.data {
            if !data.contains(elem) {
                data.push(elem.clone());
            }
        }
        CustomSet { data }
    }
}
