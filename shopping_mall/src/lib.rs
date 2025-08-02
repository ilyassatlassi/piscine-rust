mod mall;
pub use mall::*;

use core::f64;
use std::collections::HashMap;

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    let mut res = Store::new(HashMap::<String, mall::Employee>::new(), 0);
    let mut max = 0;
    let mut name = "";
    let floor = &mall.floors;
    for (_, val) in floor {
        for (key, store) in &val.stores {
            if store.square_meters > max {
                max = store.square_meters;
                // res = res.clone();
                res.employees = store.employees.clone();
                res.square_meters = store.square_meters;
                name = key;
            }
        }
    }
    return (name.to_string(), res);
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&String, &Employee)> {
    let mut res = Vec::new();
    let mut max = f64::MIN;

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for (name, employee) in &store.employees {
                if employee.salary > max {
                    res.clear();
                    res.push((name, employee));
                    max = employee.salary;
                }
                //  else if (employee.salary - max).abs() < f64::EPSILON {
                //     res.push((name, employee));
                // }
            }
        }
    }

    res
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut res = 0;
    let floor = &mall.floors;
    for (_, val) in floor {
        for (_, store) in &val.stores {
            for (key, employee) in &store.employees {
                res += 1;
            }
        }
    }
    res += mall.guards.len();
    return res;
}

pub fn check_for_securities(mall: &mut Mall, mut guard: HashMap<String, Guard>) {
    let mut total_size = 0;
    let floor = &mall.floors;
    total_size = floor.values().map(|val| val.size_limit).sum();

    let total_areas = total_size / 200;

    for (name, guard) in guard {
        if mall.guards.len() as u64 >= total_areas {
            break;
        }
        mall.hire_guard(name, guard);
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    let floor = &mut mall.floors;
    for (_, val) in floor {
        for (_, store) in &mut val.stores {
            for (key, employee) in &mut store.employees {
                let hour = (employee.working_hours.1 - employee.working_hours.0);
                if hour >= 10 {
                    employee.raise(employee.salary * 0.1);
                } else {
                    employee.cut(employee.salary * 0.1);
                }
            }
        }
    }
}
