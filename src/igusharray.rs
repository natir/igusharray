/// IgushArray struct
pub struct IgushArray<T>
where
    T: std::clone::Clone,
{
    part: Vec<std::collections::VecDeque<T>>,
    capacity: usize,
}

impl<T> IgushArray<T>
where
    T: std::clone::Clone,
{
    /// Creates an empty IgushArray.
    pub fn new() -> Self {
        IgushArray {
            part: Vec::new(),
            capacity: 0,
        }
    }

    /// Creates an empty deque with space for at least `capacity` elements.
    pub fn with_capacity(capacity: usize) -> Self {
        IgushArray {
            part: vec![
                std::collections::VecDeque::with_capacity(capacity.pow(1 / 2));
                capacity.pow(1 / 2)
            ],
            capacity: capacity,
        }
    }
}

impl<T> core::ops::Index<usize> for IgushArray<T>
where
    T: std::clone::Clone,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.part
            .index(index / self.capacity)
            .index(index % self.capacity)
    }
}

impl<T> core::ops::IndexMut<usize> for IgushArray<T>
where
    T: std::clone::Clone,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.part
            .index_mut(index / self.capacity)
            .index_mut(index % self.capacity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let _value: IgushArray<u8> = IgushArray::new();

        let _value: IgushArray<u8> = IgushArray::with_capacity(1);

        let _value: IgushArray<u8> = IgushArray::with_capacity(2);
    }
}
