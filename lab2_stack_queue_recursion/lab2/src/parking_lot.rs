use std::io::{self, BufRead};

// use the queue based on linked lists
use super::data_structures::linked_queue::LinkedList;
// use the stack based on sequential lists
use super::data_structures::array_stack::Stack;
// use the parsing utilities
use crate::parsing::Parser;
// use the shared terminal helpers
use crate::ui::{clear_screen, print_page_title, read_choice, read_usize, wait_for_enter};
// re-export the errors
pub use crate::parsing::ParseError;

const FEE_PER_TIME_UNIT: u64 = 1;

#[derive(Debug, PartialEq, Eq)]
// We expect the license to be able to be saved as a 32-bit unsigned integer.
// As for the time, it should be able to be stored as a 64-bit unsigned integer.
pub enum ParkingEvent {
    Arrival { license: u32, time: u64 },
    Departure { license: u32, time: u64 },
}

#[derive(Debug, PartialEq, Eq)]
// There are three kinds of input record in total.
// But only two of them have something to do with parking.
// The remaining one is used to signify the end of the input.
pub enum InputRecord {
    Event(ParkingEvent),
    End,
}

#[derive(Debug, PartialEq, Eq)]
enum ParkingLog {
    Parked {
        license: u32,
        position: usize,
    },
    Queued {
        license: u32,
        position: usize,
    },
    Departed {
        license: u32,
        duration: u64,
        fee: u64,
    },
}

impl std::fmt::Display for ParkingLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParkingLog::Parked { license, position } => {
                write!(f, "car {license} parks in the lot at position {position}")
            }
            ParkingLog::Queued { license, position } => {
                write!(f, "car {license} waits on the side road at position {position}")
            }
            ParkingLog::Departed {
                license,
                duration,
                fee,
            } => {
                write!(
                    f,
                    "car {license} departs: stayed for {duration} time units, fee = {fee}"
                )
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ParkingError {
    DuplicateLicense(u32),
    VehicleNotFound(u32),
    // Invariant:
    // We will reject an event whose timestamp is earlier than the
    // preceding event's timestamp
    InvalidTimestamp { previous: u64, current: u64 },
}

struct ParkingLot {
    // 1. parking stack
    parking: Stack<ParkedCar>,
    // 2. temporary stack
    temporary: Stack<ParkedCar>,
    // 3. waiting queue
    waiting: LinkedList<WaitingCar>,
    // 4. the last event's time
    last_event_time: Option<u64>,
    // 5. the parking lot's capacity
    capacity: usize,
}

struct ParkedCar {
    license: u32,
    entered_at: u64,
}

struct WaitingCar {
    license: u32,
}

struct SimulationResult {
    logs: Vec<ParkingLog>,
}


// Parse just the action
fn parse_action<R: BufRead>(p: &mut Parser<R>) -> Result<char, ParseError> {
    // ok_or converts a Option -> Result
    // next() consumes the next non-whitespace character
    let first = p.next()?.ok_or(ParseError::Expected("action"))?;

    let closing_quote = match first {
        '\'' | '"' => Some(first),
        '‘' => Some('’'),
        '“' => Some('”'),
        _ => None,
    };
    let action = if closing_quote.is_some() {
        p.next()?.ok_or(ParseError::Expected("action"))?
    } else {
        first
    };

    if let Some(quote) = closing_quote {
        p.expect(&[quote], "closing quote")?;
    }

    Ok(action)
}

// Parse one complete tuple
fn parse_record<R: BufRead>(p: &mut Parser<R>) -> Result<InputRecord, ParseError> {
    p.expect(&['(', '（'], "opening parenthesis")?;

    let action = parse_action(p)?;

    p.expect(&[',', '，'], "comma")?;

    let license = p.parse_u32()?;

    p.expect(&[',', '，'], "comma")?;

    let time = p.parse_u64()?;

    p.expect(&[')', '）'], "closing parenthesis")?;

    match action {
        'A' => Ok(InputRecord::Event(ParkingEvent::Arrival { license, time })),
        'D' => Ok(InputRecord::Event(ParkingEvent::Departure {
            license,
            time,
        })),
        'E' if license == 0 && time == 0 => Ok(InputRecord::End),
        'E' => Err(ParseError::InvalidEnd),
        // Any other action is malformed input, not an impossible state, so it
        // must surface as a parse error rather than panic the whole program.
        _ => Err(ParseError::Expected("action 'A', 'D', or 'E'")),
    }
}

// Parse the entire sequence
pub fn read_events(reader: impl BufRead) -> Result<Vec<ParkingEvent>, ParseError> {
    let mut parser = Parser::new(reader);
    let mut events = Vec::new();

    loop {
        parser.skip_ws()?;

        if parser.peek()?.is_none() {
            return Err(ParseError::MissingEnd);
        }

        match parse_record(&mut parser)? {
            InputRecord::Event(event) => events.push(event),
            InputRecord::End => return Ok(events),
        }
    }
}

fn run_simulation(lot: &mut ParkingLot, events: Vec<ParkingEvent>) -> SimulationResult {
    let logs = events
        .into_iter()
        // filter_map expects a function that returns an Option<T>
        // and it will remove the Nones and unwrap!
        .filter_map(|event| lot.handle_event(event).ok())
        .collect();
    SimulationResult { logs }
}

impl ParkingLot {
    fn new(capacity: usize) -> Self {
        Self {
            parking: Stack::new(),
            temporary: Stack::new(),
            waiting: LinkedList::new(),
            last_event_time: None,
            capacity,
        }
    }

    fn contains_parked(&mut self, license: u32) -> bool {
        let mut found = false;

        while let Some(car) = self.parking.pop() {
            // This is an elegant way of keeping updating the found
            // to see if there's finally a car whose license matches the license we search for.
            found |= car.license == license;
            self.temporary.push(car);
        }

        while let Some(car) = self.temporary.pop() {
            self.parking.push(car);
        }

        found
    }

    fn handle_event(
        &mut self,
        event: ParkingEvent,
    ) -> Result<ParkingLog, ParkingError> {
        
        // Here we simply want to extract the time field out.
        // Since it's shared by both variants (but you still cannot use event.time)
        // we use an or-pattern to extract it.
        let time = match event {
            ParkingEvent::Arrival { time, .. }
            | ParkingEvent::Departure { time, .. } => time,
        };

        if let Some(previous) = self.last_event_time {
            if time < previous {
                return Err(ParkingError::InvalidTimestamp {
                    previous,
                    current: time,
                });
            }
        }

        let log = match event {
            // Case 1 of 2: Arrival
            // we first check if there's duplicate cars with same license
            // in either the waiting queue or the parking stack.
            // the handling of them are slightly different from each other.
            ParkingEvent::Arrival { license, time } => {
                if self.waiting.iter().any(|car| car.license == license)
                    || self.contains_parked(license)
                {
                    return Err(ParkingError::DuplicateLicense(license));
                }

                if self.parking.len() < self.capacity {
                    self.parking.push(ParkedCar {
                        license,
                        entered_at: time,
                    });

                    // return the result (Parked variant of the log)
                    ParkingLog::Parked {
                        license,
                        position: self.parking.len(),
                    }
                } else {
                    self.waiting.push(WaitingCar { license });

                    // return the result (Queued variant of the log)
                    ParkingLog::Queued {
                        license,
                        position: self.waiting.len(),
                    }
                }
            }

            ParkingEvent::Departure { license, time } => {
                let mut leaving = None;

                while let Some(car) = self.parking.pop() {
                    if car.license == license {
                        leaving = Some(car);
                        break;
                    }

                    self.temporary.push(car);
                }

                while let Some(car) = self.temporary.pop() {
                    self.parking.push(car);
                }

                let leaving =
                    leaving.ok_or(ParkingError::VehicleNotFound(license))?;

                // There's one car that leaves, so correspondingly there's one
                // car that's currently queueing which can enter.
                if let Some(next) = self.waiting.pop() {
                    self.parking.push(ParkedCar {
                        license: next.license,
                        entered_at: time,
                    });
                }

                let duration = time - leaving.entered_at;

                ParkingLog::Departed {
                    license,
                    duration,
                    fee: duration * FEE_PER_TIME_UNIT,
                }
            }
        };

        self.last_event_time = Some(time);
        Ok(log)
    }
}

pub fn run() {
    loop {
        clear_screen();
        print_page_title("Parking Lot");

        println!("  1. Run a simulation");
        println!("  0. Return to the main menu");
        println!();

        match read_choice() {
            1 => run_simulation_once(),
            0 => return,
            _ => println!("Invalid choice — please enter 0 or 1."),
        }

        wait_for_enter();
    }
}

// Reads the capacity and the event records from stdin, runs the simulation and
// prints the resulting log.
fn run_simulation_once() {
    let capacity = read_usize("Please enter the parking lot capacity n: ");

    println!();
    println!("Please enter the events, one per line, as (A/D, license, time),");
    println!("and finish with (E, 0, 0):");

    let events = match read_events(io::stdin().lock()) {
        Ok(events) => events,
        Err(error) => {
            println!();
            println!("  Failed to parse the input: {error}");
            return;
        }
    };

    let mut lot = ParkingLot::new(capacity);
    let result = run_simulation(&mut lot, events);

    println!();
    println!("  Simulation result:");
    for log in &result.logs {
        println!("    - {log}");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    fn parse(input: &str) -> Result<Vec<ParkingEvent>, ParseError> {
        read_events(Cursor::new(input.as_bytes()))
    }

    fn lot(capacity: usize) -> ParkingLot {
        ParkingLot::new(capacity)
    }

    #[test]
    fn read_events_parses_a_valid_sequence() {
        let events =
            parse("(A, 1001, 0)\n(A, 1002, 1)\n(D, 1001, 5)\n(E, 0, 0)\n").unwrap();
        assert_eq!(
            events,
            vec![
                ParkingEvent::Arrival { license: 1001, time: 0 },
                ParkingEvent::Arrival { license: 1002, time: 1 },
                ParkingEvent::Departure { license: 1001, time: 5 },
            ]
        );
    }

    #[test]
    fn read_events_accepts_quotes_and_fullwidth_punctuation() {
        let events = parse("（'A'，1001，0）\n(\"D\", 1001, 5)\n(E, 0, 0)\n").unwrap();
        assert_eq!(
            events,
            vec![
                ParkingEvent::Arrival { license: 1001, time: 0 },
                ParkingEvent::Departure { license: 1001, time: 5 },
            ]
        );
    }

    #[test]
    fn read_events_requires_an_end_record() {
        assert!(matches!(parse("(A, 1001, 0)\n"), Err(ParseError::MissingEnd)));
    }

    #[test]
    fn read_events_rejects_a_malformed_end_record() {
        assert!(matches!(parse("(E, 1, 2)\n"), Err(ParseError::InvalidEnd)));
    }

    #[test]
    fn read_events_rejects_an_unknown_action() {
        assert!(matches!(
            parse("(X, 1, 2)\n(E, 0, 0)\n"),
            Err(ParseError::Expected(_))
        ));
    }

    #[test]
    fn read_events_rejects_an_oversized_license() {
        assert!(matches!(
            parse("(A, 4294967296, 0)\n(E, 0, 0)\n"),
            Err(ParseError::NumberTooLarge)
        ));
    }

    #[test]
    fn arrival_parks_when_space_is_available() {
        let mut lot = lot(2);
        let log = lot
            .handle_event(ParkingEvent::Arrival { license: 1, time: 0 })
            .unwrap();
        assert_eq!(log, ParkingLog::Parked { license: 1, position: 1 });
    }

    #[test]
    fn arrival_queues_when_lot_is_full() {
        let mut lot = lot(1);
        lot.handle_event(ParkingEvent::Arrival { license: 1, time: 0 })
            .unwrap();
        let log = lot
            .handle_event(ParkingEvent::Arrival { license: 2, time: 0 })
            .unwrap();
        assert_eq!(log, ParkingLog::Queued { license: 2, position: 1 });
    }

    #[test]
    fn departure_charges_fee_for_time_parked() {
        let mut lot = lot(1);
        lot.handle_event(ParkingEvent::Arrival { license: 1, time: 2 })
            .unwrap();
        let log = lot
            .handle_event(ParkingEvent::Departure { license: 1, time: 7 })
            .unwrap();
        assert_eq!(log, ParkingLog::Departed { license: 1, duration: 5, fee: 5 });
    }

    #[test]
    fn departure_promotes_the_waiting_car() {
        let mut lot = lot(1);
        lot.handle_event(ParkingEvent::Arrival { license: 1, time: 0 })
            .unwrap();
        lot.handle_event(ParkingEvent::Arrival { license: 2, time: 0 })
            .unwrap();

        let leaving = lot
            .handle_event(ParkingEvent::Departure { license: 1, time: 5 })
            .unwrap();
        assert_eq!(leaving, ParkingLog::Departed { license: 1, duration: 5, fee: 5 });

        // Car 2 was parked at time 5 (the departure time), so leaving at time 9
        // costs 4 units.
        let promoted = lot
            .handle_event(ParkingEvent::Departure { license: 2, time: 9 })
            .unwrap();
        assert_eq!(promoted, ParkingLog::Departed { license: 2, duration: 4, fee: 4 });
    }

    #[test]
    fn duplicate_parked_license_is_rejected() {
        let mut lot = lot(2);
        lot.handle_event(ParkingEvent::Arrival { license: 1, time: 0 })
            .unwrap();
        let result = lot.handle_event(ParkingEvent::Arrival { license: 1, time: 1 });
        assert_eq!(result, Err(ParkingError::DuplicateLicense(1)));
    }

    #[test]
    fn duplicate_queued_license_is_rejected() {
        let mut lot = lot(1);
        lot.handle_event(ParkingEvent::Arrival { license: 1, time: 0 })
            .unwrap();
        lot.handle_event(ParkingEvent::Arrival { license: 2, time: 0 })
            .unwrap();
        let result = lot.handle_event(ParkingEvent::Arrival { license: 2, time: 1 });
        assert_eq!(result, Err(ParkingError::DuplicateLicense(2)));
    }

    #[test]
    fn departing_an_unknown_car_is_an_error() {
        let mut lot = lot(2);
        let result = lot.handle_event(ParkingEvent::Departure { license: 99, time: 0 });
        assert_eq!(result, Err(ParkingError::VehicleNotFound(99)));
    }

    #[test]
    fn out_of_order_timestamp_is_rejected() {
        let mut lot = lot(2);
        lot.handle_event(ParkingEvent::Arrival { license: 1, time: 5 })
            .unwrap();
        let result = lot.handle_event(ParkingEvent::Arrival { license: 2, time: 3 });
        assert_eq!(
            result,
            Err(ParkingError::InvalidTimestamp { previous: 5, current: 3 })
        );
    }

    #[test]
    fn full_simulation_produces_the_expected_log() {
        let mut lot = lot(2);
        let events = vec![
            ParkingEvent::Arrival { license: 1, time: 0 },
            ParkingEvent::Arrival { license: 2, time: 0 },
            ParkingEvent::Arrival { license: 3, time: 1 },
            ParkingEvent::Departure { license: 2, time: 5 },
            ParkingEvent::Departure { license: 1, time: 8 },
            ParkingEvent::Departure { license: 3, time: 10 },
        ];
        let result = run_simulation(&mut lot, events);

        assert_eq!(
            result.logs,
            vec![
                ParkingLog::Parked { license: 1, position: 1 },
                ParkingLog::Parked { license: 2, position: 2 },
                ParkingLog::Queued { license: 3, position: 1 },
                ParkingLog::Departed { license: 2, duration: 5, fee: 5 },
                ParkingLog::Departed { license: 1, duration: 8, fee: 8 },
                ParkingLog::Departed { license: 3, duration: 5, fee: 5 },
            ]
        );
    }
}