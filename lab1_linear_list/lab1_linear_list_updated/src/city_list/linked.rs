use crate::ui::{clear_screen, print_page_title, wait_for_enter};
use std::io::{self, Write};
// Invariant:
// - city names must be unique;
// - indexing is zero-based;
// - insert_at(len, city) is allowed and appends;
// - insert_at(index > len, city) is out of bounds;
// - remove_at(index >= len) returns None;
// - distance search returns cities in their current list order.
#[derive(Debug, PartialEq, Clone)]
struct City {
    name: String,
    point: Point,
}

#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, PartialEq, Clone)]
enum ListError {
    DuplicateCityName,
    IndexOutOfBounds,
}

struct CityNode {
    // `city` is `None` only for the dummy head node (带头结点); every real node
    // holds `Some(city)`.
    city: Option<City>,
    next: Option<Box<CityNode>>,
}

struct CityLinkedList {
    // Dummy head node. `head.next` points to the first real node, or `None` when
    // the list is empty.
    head: Box<CityNode>,
}

impl City {
    fn within_distance_of(&self, compare: &Point, distance: f64) -> bool {
        (self.point.x - compare.x).powi(2) + (self.point.y - compare.y).powi(2) <= distance.powi(2)
    }
}

impl CityLinkedList {
    fn new() -> Self {
        CityLinkedList {
            head: Box::new(CityNode {
                city: None,
                next: None,
            }),
        }
    }

    // insertion
    // insert at the i-th position
    // means that the new element becomes the i-th element
    // we call the first real node the 0-th node
    fn insert_at(&mut self, index: usize, city: City) -> Result<(), ListError> {
        if self.find_by_name(&city.name).is_some() {
            return Err(ListError::DuplicateCityName);
        }
        let mut next = &mut self.head.next;

        for _ in 0..index {
            let Some(ref_ptr_next) = next else {
                return Err(ListError::IndexOutOfBounds);
            };
            next = &mut ref_ptr_next.next;
        }

        *next = Some(Box::new(CityNode {
            city: Some(city),
            next: next.take(),
        }));

        Ok(())
    }

    // deletion
    fn remove_at(&mut self, index: usize) -> Option<City> {
        let mut next = &mut self.head.next;

        for _ in 0..index {
            let Some(ref_ptr_next) = next else {
                return None;
            };
            next = &mut ref_ptr_next.next;
        }

        let mut node = next.take()?;
        *next = node.next.take();
        node.city.take()
    }

    //  [Note] There's something wrong with the following, commented-out version
    //
    //         It has both an immutable and a mutable RE-borrow of the mutable borrow, which is the
    //         problem.
    //    // modification
    //    fn update_at(&mut self, index: usize, city: City) -> Result<(), ListError> {
    //        // Invariant:
    //        //     There cannot be two cities in the list that have the same name
    //        //     How might a user attempt to break this invariant:
    //        //          1. Suppose the user wants to update the city, with it's name unchanged, that's OK
    //        //          2. The user wants to update the city and change its name
    //        //              a. There have been a city with this name -> Problem!
    //        //              b. There haven't been any city of this name -> Ok!
    //        //     So we check under the condition that the city's name has been changed, that
    //        //     if there's any existing city with that name!
    //        let mut next = &mut self.head;
    //
    //        for _ in 1..=index {
    //            let Some(ref_ptr_next) = next else {
    //                return Err(ListError::IndexOutOfBounds);
    //            };
    //            next = &mut ref_ptr_next.next;
    //        }
    //
    //        if let Some(to_modify_ptr) = next {
    //            if city.name != to_modify_ptr.city.name && self.find_by_name(&city.name).is_some() {
    //                return Err(ListError::DuplicateCityName);
    //            }
    //            *next = Some(Box::new(CityNode {
    //                city,
    //                next: to_modify_ptr.next.take(),
    //            }));
    //            Ok(())
    //        } else {
    //            Err(ListError::IndexOutOfBounds)
    //        }
    //    }
    fn update_at(&mut self, index: usize, city: City) -> Result<(), ListError> {
        // ----- Phase 1: immutable inspection -----

        let mut current = self.head.next.as_ref();

        for _ in 0..index {
            let Some(node) = current else {
                return Err(ListError::IndexOutOfBounds);
            };

            current = node.next.as_ref();
        }

        let Some(current_node) = current else {
            return Err(ListError::IndexOutOfBounds);
        };

        let current_city = current_node
            .city
            .as_ref()
            .expect("real node always holds a city");

        if city.name != current_city.name && self.find_by_name(&city.name).is_some() {
            return Err(ListError::DuplicateCityName);
        }

        // ----- Phase 2: mutable modification -----

        let mut current = self.head.next.as_mut();

        for _ in 0..index {
            current = current.unwrap().next.as_mut();
        }

        current.unwrap().city = Some(city);

        Ok(())
    }
    // search
    fn find_by_name(&self, name: &str) -> Option<&City> {
        let mut next = self.head.next.as_ref();
        while let Some(ptr_next) = next {
            if let Some(city) = &ptr_next.city
                && city.name == name
            {
                return Some(city);
            }
            next = ptr_next.next.as_ref();
        }
        None
    }

    fn cities_within_distance_of(&self, point: &Point, max_distance: f64) -> Vec<&City> {
        let mut next = self.head.next.as_ref();
        let mut cities: Vec<&City> = Vec::new();
        while let Some(ptr_next) = next {
            if let Some(city) = &ptr_next.city
                && city.within_distance_of(point, max_distance)
            {
                cities.push(city);
            }
            next = ptr_next.next.as_ref();
        }
        cities
    }
}
pub fn run() {
    let mut list = CityLinkedList::new();

    loop {
        clear_screen();
        print_page_title("City List — Linked List");
        println!("  1. Insert city at index");
        println!("  2. Remove city at index");
        println!("  3. Update city at index");
        println!("  4. Find city by name");
        println!("  5. Find cities within distance of a point");
        println!("  6. Display all cities");
        println!("  0. Return to main menu");
        println!();

        match read_usize("Please enter your choice: ") {
            1 => {
                let index = read_usize("Index: ");
                let city = read_city();

                match list.insert_at(index, city) {
                    Ok(()) => println!("City inserted successfully."),
                    Err(ListError::DuplicateCityName) => {
                        println!("Error: a city with that name already exists.")
                    }
                    Err(ListError::IndexOutOfBounds) => {
                        println!("Error: index is out of bounds.")
                    }
                }
            }

            2 => {
                let index = read_usize("Index: ");

                match list.remove_at(index) {
                    Some(city) => {
                        println!("Removed city:");
                        print_city(&city);
                    }
                    None => println!("Error: index is out of bounds."),
                }
            }

            3 => {
                let index = read_usize("Index: ");

                println!("Enter the new city information:");
                let city = read_city();

                match list.update_at(index, city) {
                    Ok(()) => println!("City updated successfully."),
                    Err(ListError::DuplicateCityName) => {
                        println!("Error: a city with that name already exists.")
                    }
                    Err(ListError::IndexOutOfBounds) => {
                        println!("Error: index is out of bounds.")
                    }
                }
            }

            4 => {
                let name = read_string("City name: ");

                match list.find_by_name(&name) {
                    Some(city) => {
                        println!("City found:");
                        print_city(city);
                    }
                    None => println!("No city named \"{name}\" was found."),
                }
            }

            5 => {
                println!("Enter the center point:");

                let x = read_f64("x: ");
                let y = read_f64("y: ");

                let max_distance = loop {
                    let distance = read_f64("Maximum distance: ");

                    if distance >= 0.0 {
                        break distance;
                    }

                    println!("Distance must be non-negative.");
                };

                let point = Point { x, y };
                let cities = list.cities_within_distance_of(&point, max_distance);

                if cities.is_empty() {
                    println!("No cities were found within that distance.");
                } else {
                    println!("Cities found:");

                    for (index, city) in cities.iter().enumerate() {
                        println!(
                            "  {}. {} ({}, {})",
                            index + 1,
                            city.name,
                            city.point.x,
                            city.point.y
                        );
                    }
                }
            }

            6 => {
                print_linked_list(&list);
            }

            0 => break,

            _ => {
                println!("Invalid choice. Please enter a number from 0 to 6.");
            }
        }

        wait_for_enter();
    }
}

fn print_linked_list(list: &CityLinkedList) {
    println!();

    let mut current = list.head.next.as_ref();
    let mut index = 0;

    if current.is_none() {
        println!("The list is empty.");
        return;
    }

    println!("Cities:");

    while let Some(node) = current {
        if let Some(city) = &node.city {
            println!(
                "  [{index}] {} ({}, {})",
                city.name, city.point.x, city.point.y
            );
        }

        current = node.next.as_ref();
        index += 1;
    }
}

fn print_city(city: &City) {
    println!("{} ({}, {})", city.name, city.point.x, city.point.y);
}

fn read_city() -> City {
    let name = read_string("City name: ");
    let x = read_f64("x: ");
    let y = read_f64("y: ");

    City {
        name,
        point: Point { x, y },
    }
}

fn read_string(prompt: &str) -> String {
    loop {
        print!("{prompt}");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                if !input.is_empty() {
                    return input.to_owned();
                }

                println!("Input cannot be empty.");
            }

            Err(error) => {
                eprintln!("Failed to read input: {error}");
            }
        }
    }
}

fn read_usize(prompt: &str) -> usize {
    loop {
        let input = read_string(prompt);

        match input.parse::<usize>() {
            Ok(value) => return value,
            Err(_) => println!("Please enter a valid non-negative integer."),
        }
    }
}

fn read_f64(prompt: &str) -> f64 {
    loop {
        let input = read_string(prompt);

        match input.parse::<f64>() {
            Ok(value) if value.is_finite() => return value,
            Ok(_) => println!("Please enter a finite number."),
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // we need to test the following functions
    // City:
    //   within_distance_of
    #[test]
    fn test_city_distance_fn() {
        let test_city: City = City {
            name: String::from("Chongqing"),
            point: Point { x: 0f64, y: 0f64 },
        };
        let max_distance: f64 = 5.0;
        let point1 = Point { x: 2.0, y: 4.0 };
        let point2 = Point { x: 3.0, y: 4.0 };
        let point3 = Point { x: 3.0, y: 5.0 };
        assert!(test_city.within_distance_of(&point1, max_distance));
        assert!(test_city.within_distance_of(&point2, max_distance));
        assert!(!test_city.within_distance_of(&point3, max_distance));
    }
    // we need to test the following functions
    // CityLinkedList:
    //   new
    //   insert_at
    //   remove_at
    //   update_at
    //   find_by_name
    //   cities_within_distance_of
    #[test]
    fn test_linked_list() {
        let mut test_list = CityLinkedList::new();
        let chongqing: City = City {
            name: String::from("Chongqing"),
            point: Point { x: 0f64, y: 0f64 },
        };
        let sichuan: City = City {
            name: String::from("Sichuan"),
            point: Point { x: 12f64, y: 5f64 },
        };
        let shanghai: City = City {
            name: String::from("Shanghai"),
            point: Point { x: 3f64, y: 4f64 },
        };
        let beijing: City = City {
            name: String::from("Beijing"),
            point: Point { x: 7f64, y: 24f64 },
        };
        assert_eq!(Ok(()), test_list.insert_at(0, chongqing.clone()));
        assert_eq!(test_list.remove_at(0), Some(chongqing.clone()));
        assert_eq!(test_list.remove_at(0), None);

        // only Chongqing
        assert_eq!(Ok(()), test_list.insert_at(0, chongqing.clone()));
        assert_eq!(
            Err(ListError::DuplicateCityName),
            test_list.insert_at(0, chongqing.clone())
        );
        assert_eq!(test_list.find_by_name("Chongqing"), Some(&chongqing));

        // Beijing, Chongqing
        assert_eq!(Ok(()), test_list.insert_at(0, beijing.clone()));
        assert_eq!(test_list.find_by_name("Beijing"), Some(&beijing));

        // Shanghai, Beijing, Chongqing
        assert_eq!(Ok(()), test_list.insert_at(0, shanghai.clone()));

        // Beijing, Chongqing
        assert_eq!(Some(shanghai.clone()), test_list.remove_at(0));
        assert_eq!(None, test_list.remove_at(2));

        // Beijing
        assert_eq!(Some(beijing.clone()), test_list.remove_at(0));

        // Chongqing
        assert_eq!(Some(chongqing.clone()), test_list.remove_at(0));

        // Nothing here!!

        // Build [shanghai, sichuan, chongqing, beijing] by inserting at the front.
        assert_eq!(Ok(()), test_list.insert_at(0, beijing.clone()));
        assert_eq!(Ok(()), test_list.insert_at(0, chongqing.clone()));
        assert_eq!(Ok(()), test_list.insert_at(0, sichuan.clone()));
        assert_eq!(Ok(()), test_list.insert_at(0, shanghai.clone()));

        assert_eq!(
            test_list.cities_within_distance_of(&Point { x: 0f64, y: 0f64 }, 13f64),
            vec![&shanghai, &sichuan, &chongqing]
        );

        let chongqing_1 = City {
            name: String::from("Chongqing1"),
            ..chongqing.clone()
        };
        let beijing_1 = City {
            name: String::from("Beijing1"),
            ..beijing.clone()
        };

        let _ = test_list.update_at(2, chongqing_1.clone());
        let _ = test_list.update_at(3, beijing_1.clone());

        assert_eq!(
            test_list.cities_within_distance_of(&Point { x: 0f64, y: 0f64 }, 13f64),
            vec![&shanghai, &sichuan, &chongqing_1]
        );
    }
}
