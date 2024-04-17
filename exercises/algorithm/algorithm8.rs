/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

#[allow(non_camel_case_types)]
pub struct myStack<T>
{
    active_q1: bool,
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			active_q1: true,
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        let active = match self.active_q1 {
            true => &mut self.q1,
            false => &mut self.q2,
        };
        active.enqueue(elem);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        let (active, inactive) = match self.active_q1 {
            true => (&mut self.q1, &mut self.q2),
            false => (&mut self.q2, &mut self.q1)
        };

        if active.is_empty() {
            return Err("Stack is empty");
        }

        while let Ok(e) = active.dequeue() {
            if active.is_empty() {
                self.active_q1 = !self.active_q1;
                return Ok(e);
            } else {
                inactive.enqueue(e);
            }
        }
        unreachable!();
    }
    fn get_active(&self) -> &Queue<T> {
        if self.active_q1 {
            &self.q1
        } else {
            &self.q2
        }
    }
    pub fn is_empty(&self) -> bool {
        self.get_active().is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}
