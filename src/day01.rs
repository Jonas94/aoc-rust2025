use std::cell::RefCell;
use std::char;
use std::rc::Rc;

pub fn part1(input: &str) -> i32 {
    let mut hit_zero: i32 = 0;

    let mut nodes: Vec<Rc<RefCell<Node<i32>>>> = Vec::new();

    // Create 100 nodes with no next pointers yet
    for i in 0..=99 {
        nodes.push(Rc::new(RefCell::new(Node {
            value: i,
            prev: None,
            next: None,
        })));
    }

    // Link each node to the next one
    for i in 0..99 {
        let next = Rc::clone(&nodes[i + 1]);
        nodes[i].borrow_mut().next = Some(next);
        nodes[i + 1].borrow_mut().prev = Some(Rc::clone(&nodes[i]));
    }

    // Close the circle: last node points back to the first
    nodes[99].borrow_mut().next = Some(Rc::clone(&nodes[0]));
    let mut current = Rc::clone(&nodes[50]);

    nodes[0].borrow_mut().prev = Some(Rc::clone(&nodes[99]));

    for line in input.lines() {
        if let Some((letter, number)) = split_code(line) {
            println!("line: {}, {}", letter, number);

            if letter == 'L' {
                for _ in 0..(number as usize) {
                    let prev_node = {
                        let b = current.borrow();
                        b.prev.as_ref().unwrap().clone()
                    };
                    current = prev_node;
                }
            } else if letter == 'R' {
                for _ in 0..(number as usize) {
                    let next_node = {
                        let b = current.borrow();
                        b.next.as_ref().unwrap().clone()
                    };
                    current = next_node;
                }
            }
            //    println!("currentIndex: {}", next.as_ref().borrow().value);

            let val = current.borrow().value;

            if val == 0 {
                hit_zero = hit_zero + 1;
            }
        }
    }
    return hit_zero;
}

fn split_code(s: &str) -> Option<(char, i32)> {
    let mut chars = s.chars();
    let letter = chars.next()?;
    let rest: String = chars.collect();

    let number: i32 = rest.parse::<i32>().ok()?;
    Some((letter, number))
}

pub fn part2(input: &str) -> i32 {
    let mut hit_zero: i32 = 0;

    let mut nodes: Vec<Rc<RefCell<Node<i32>>>> = Vec::new();

    // Create 100 nodes with no next pointers yet
    for i in 0..=99 {
        nodes.push(Rc::new(RefCell::new(Node {
            value: i,
            prev: None,
            next: None,
        })));
    }

    // Link each node to the next one
    for i in 0..99 {
        let next = Rc::clone(&nodes[i + 1]);
        nodes[i].borrow_mut().next = Some(next);
        nodes[i + 1].borrow_mut().prev = Some(Rc::clone(&nodes[i]));
    }

    // Close the circle: last node points back to the first
    nodes[99].borrow_mut().next = Some(Rc::clone(&nodes[0]));
    let mut current = Rc::clone(&nodes[50]);

    nodes[0].borrow_mut().prev = Some(Rc::clone(&nodes[99]));

    for line in input.lines() {
        if let Some((letter, number)) = split_code(line) {
            if letter == 'L' {
                for _ in 0..(number as usize) {
                    let prev_node = {
                        let b = current.borrow();
                        b.prev.as_ref().unwrap().clone()
                    };
                    current = prev_node;

                    let val = current.borrow().value;

                    if val == 0 {
                        hit_zero = hit_zero + 1;
                    }
                }
            } else if letter == 'R' {
                for _ in 0..(number as usize) {
                    let next_node = {
                        let b = current.borrow();
                        b.next.as_ref().unwrap().clone()
                    };
                    current = next_node;

                    let val = current.borrow().value;

                    if val == 0 {
                        hit_zero = hit_zero + 1;
                    }
                }
            }
            //    println!("currentIndex: {}", next.as_ref().borrow().value);

            //           if next.as_ref(). == 0 {
            //             hit_zero = hit_zero + 1;
            //       }
        }
    }
    return hit_zero;
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    value: T,
    prev: Link<T>,
    next: Link<T>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1test() {
        assert_eq!(
            part1(
                "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
            ),
            3
        );
    }

    #[test]
    fn part2test() {
        assert_eq!(
            part2(
                "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
            ),
            6
        );
    }
}
