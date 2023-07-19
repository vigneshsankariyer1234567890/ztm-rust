// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * Corporate must be able to access the rentals at a storefront
// * Storefronts must be able to rent out vehicles
// * Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate

use std::{rc::Rc, cell::RefCell};

struct Corporate(Rentals);

struct StoreFront(Rentals);

type Rentals = Rc<RefCell<Vec<Rental>>>;
#[derive(Debug)]
struct Rental {
  vehicle_type: VehicleType,
  vin: VehicleIdentificationNumber,
  status: VehicleStatus,
}

#[derive(Debug)]
enum VehicleType {
  Sedan,
  HatchBack,
  SUV,
  Luxury,
}

#[derive(Debug)]
struct VehicleIdentificationNumber(u16);

#[derive(Debug)]
enum VehicleStatus {
  Available,
  Unavailable,
  Maintenance,
  Rented
}

fn main() {
  let rentals = Rc::new(RefCell::new(vec![
    Rental { vehicle_type: VehicleType::HatchBack, vin: VehicleIdentificationNumber(0), status: VehicleStatus::Available},
    Rental { vehicle_type: VehicleType::Sedan, vin: VehicleIdentificationNumber(1), status: VehicleStatus::Unavailable},
    Rental { vehicle_type: VehicleType::SUV, vin: VehicleIdentificationNumber(2), status: VehicleStatus::Maintenance},
    Rental { vehicle_type: VehicleType::Luxury, vin: VehicleIdentificationNumber(3), status: VehicleStatus::Rented},
  ]));

  let store_front = StoreFront(Rc::clone(&rentals));

  let corporate = Corporate(Rc::clone(&rentals));

  dbg!(store_front.0.borrow());
  dbg!(corporate.0.borrow());

  {
    corporate.0.borrow_mut().push(Rental { vehicle_type: VehicleType::Luxury, vin: VehicleIdentificationNumber(4), status: VehicleStatus::Available});
  }

  dbg!(store_front.0.borrow());
  dbg!(corporate.0.borrow());

  {
    store_front.0.borrow_mut()[0].status = VehicleStatus::Available;
  }

  dbg!(store_front.0.borrow());
  dbg!(corporate.0.borrow());

  drop(corporate);

  dbg!(store_front.0.borrow());
}
