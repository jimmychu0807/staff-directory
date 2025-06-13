# Staff Directory Application in Rust

## Features
- ask for the name of the team
- can save and load to a json file
- list the department
- list the staff directory
- CUD a department name
- CUD a staff

## Data Structure

Department

```rs
// departmentId start from 000
type DepartmentId = String;

struct Department {
  id: DepartmentId,
  name: String,
  parent: Option<DepartmentId>
}
```

Staff

```rs
// staffId start from 00000
type StaffId = String;
enum Gender {
  Male,
  Female
}

struct Staff {
  id: StaffId
  first_name: String,
  last_name: String,
  email: String,
  ranking: u32,
  age: u32,
  gender: Gender,
  department: Option<DepartmentId>,
  salary: u32,
}
```

## Application Menu

```
What do you want to do?
1. List all staff (alphabetically)    - [ls]
2. Create a new staff                 - [cs]
3. Retrieve a staff info              - [rs]
  - what is the staffId, first name, last name, or email
  - found the following, which particular staff info do you want to retrieve
  - list the staff full info
4. Update a staff info                - [us]
5. Delete an existing staff           - [ds]
  - what is the staffId, first name, last name, or email
  - based on your info, we found ... which one do you want to remove,
6. List department hierarchy          - [ld]
7. Insert a new department            - [cd]
8. Retrieve a department info         - [rd]
9. Update an existing department info - [ud]
10. Remove an existing department     - [dd]
11. Open from a file                  - [o]
12. Save to a file                    - [s]
13. Exit                              - [q]
```

- Pressing `Esc` key can always cancel the action and go back one level up.

# Questions to think about

- Currently you are using trait object to implement menu_item and its execution. Will it be better to use a struct type directly to implement menu_item and a function pointer that accept different kind of execution implementation?
