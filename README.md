# Employee Schedule Management System

## Overview

This project implements an employee scheduling application in two different programming languages: Go and Rust. The application demonstrates the use of control structures such as conditionals, loops, branching, and data structures while managing employee work schedules for a seven-day work week.

The company operates every day of the week and provides three shifts per day:

* Morning
* Afternoon
* Evening

The program automatically assigns employees to shifts while following scheduling constraints and resolving conflicts when necessary.

---

# Project Objectives

The main objective of this project is to demonstrate :

* Conditional statements
* Loops and iteration
* Branching logic
* Arrays, slices, vectors, and hash maps
* Control flow in multiple programming paradigms
* Conflict resolution logic
* Structured output formatting

---

# Features

## Employee Input and Storage

The application stores:

* Employee names
* Preferred shifts
* Number of days worked
* Daily work assignments

The data is stored using appropriate data structures in both Go and Rust implementations.

---

# Scheduling Rules

The scheduler follows these constraints:

* An employee cannot work more than one shift per day.
* An employee can work a maximum of five days per week.
* Each shift must contain at least two employees.
* If a shift does not have enough preferred employees, the program automatically assigns available employees.
* Scheduling conflicts are resolved by assigning employees to another available shift.

---

# Languages Used

This project was implemented using:

* Go
* Rust

These languages were selected because they provide a strong contrast in syntax, memory management, and programming paradigms.

---

# Project Structure

```text
Assignment 4- Employee Scheduler/
│
├── go-version/
│   └── main.go
│
├── rust-version/
│   └── src/
│       └── main.rs
```

---

# Sample Employees

The following sample employees are included in the program:

* Alice
* Bob
* Charlie
* Diana
* Ethan
* Fiona
* George
* Hannah
* Ian

Each employee is assigned a preferred shift.

---

# How to Run the Go Version

## Step 1: Navigate to the Go folder

```bash
cd go-version
```

## Step 2: Run the program

```bash
go run main.go
```

---

# How to Run the Rust Version

## Step 1: Navigate to the Rust folder

```bash
cd rust-version
```

## Step 2: Run the program

```bash
cargo run
```

---

# Author

Name: Prashanna Acharya

Course: Advanced Programming Languages (MSCS 632-A01)

Assignment: Employee Schedule Management System

Professor: Dr. Jay Thom
