# Staff Directory Application in Rust

Features:
- ask for the name of the team
- can save and load to a json file
- list the department
- list the staff directory
- CUD a department name
- CUD a staff

department data structure

```rs
// departmentId start from 000
type DepartmentId = String;

struct Department {
  id: DepartmentId,
  name: String,
  parent: Option<DepartmentId>
}
```

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
  salary: u64,
}
```

The application menu

```
What do you want to do?
1. List department hierarchy
2. Insert a new staff
3. Retrieve a staff info
  - what is the staffId, first name, last name, or email
  - found the following, which particular staff info do you want to retrieve
  - list the staff full info
4. Update a staff info
5. Remove an existing staff
  - what is the staffId, first name, last name, or email
  - based on your info, we found ... which one do you want to remove,
6. List all staff (alphabetically)
7. Insert a new department
8. Retrieve a department info
9. Update an existing department info
10. Remove an existing department
11. Load from a file
12. Save to a file
13. Exit [q]
```

- Pressing `Esc` key can always cancel the action and go back one level up.
