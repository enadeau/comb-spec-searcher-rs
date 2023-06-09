use crate::combinatorial_class::CombinatorialClass;

pub struct ClassDB<C: CombinatorialClass> {
    data: Vec<C>,
}

impl<C: CombinatorialClass> ClassDB<C> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn get_label_from_class_or_add(&mut self, class: &C) -> usize {
        match self.get_label_from_class(class) {
            Some(index) => index,
            None => {
                self.data.push(class.clone());
                self.data.len() - 1
            }
        }
    }

    pub fn get_label_from_class(&self, class: &C) -> Option<usize> {
        self.data.iter().position(|x| x == class)
    }

    pub fn get_class_from_label(&self, label: usize) -> Option<&C> {
        self.data.get(label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::word::AvoidingWithPrefix;

    #[test]
    fn get_label_test() {
        let alphabet = vec!['a', 'b'];
        let patterns = vec![String::from("aaa")];
        let w1 = AvoidingWithPrefix::new(String::from(""), patterns.clone(), alphabet.clone());
        let w2 = AvoidingWithPrefix::new(String::from("a"), patterns.clone(), alphabet.clone());
        let w3 = AvoidingWithPrefix::new(String::from("b"), patterns.clone(), alphabet.clone());
        let mut classdb = ClassDB::new();
        assert_eq!(classdb.get_label_from_class(&w1), None);
        assert_eq!(classdb.get_label_from_class_or_add(&w1), 0);
        assert_eq!(classdb.get_label_from_class(&w1), Some(0));
        assert_eq!(classdb.get_label_from_class_or_add(&w2), 1);
        assert_eq!(classdb.get_label_from_class(&w1), Some(0));
        assert_eq!(classdb.get_label_from_class(&w2), Some(1));
        assert_eq!(classdb.get_label_from_class_or_add(&w3), 2);
        assert_eq!(classdb.get_label_from_class(&w1), Some(0));
        assert_eq!(classdb.get_label_from_class(&w2), Some(1));
    }
}
