package main

import (
	"fmt"
	"math/rand"
	"time"
)

var days = []string{"Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"}
var shifts = []string{"Morning", "Afternoon", "Evening"}

type Employee struct {
	Name       string
	Preferred  string
	DaysWorked int
	WorkedDays map[string]bool
}

func main() {
	rand.Seed(time.Now().UnixNano())

	employees := []Employee{
		{"Alice", "Morning", 0, map[string]bool{}},
		{"Bob", "Morning", 0, map[string]bool{}},
		{"Charlie", "Afternoon", 0, map[string]bool{}},
		{"Diana", "Afternoon", 0, map[string]bool{}},
		{"Ethan", "Evening", 0, map[string]bool{}},
		{"Fiona", "Evening", 0, map[string]bool{}},
		{"George", "Morning", 0, map[string]bool{}},
		{"Hannah", "Afternoon", 0, map[string]bool{}},
		{"Ian", "Evening", 0, map[string]bool{}},
	}

	schedule := make(map[string]map[string][]string)

	for _, day := range days {
		schedule[day] = make(map[string][]string)

		for _, shift := range shifts {
			schedule[day][shift] = []string{}
		}
	}

	for dayIndex, day := range days {
		for shiftIndex, shift := range shifts {
			for i := range employees {
				if len(schedule[day][shift]) >= 2 {
					break
				}

				if employees[i].Preferred == shift &&
					employees[i].DaysWorked < 5 &&
					!employees[i].WorkedDays[day] {

					schedule[day][shift] = append(schedule[day][shift], employees[i].Name)
					employees[i].DaysWorked++
					employees[i].WorkedDays[day] = true
				}
			}

			for len(schedule[day][shift]) < 2 {
				assigned := false
				randomOrder := rand.Perm(len(employees))

				for _, employeeIndex := range randomOrder {
					if employees[employeeIndex].DaysWorked < 5 &&
						!employees[employeeIndex].WorkedDays[day] {

						schedule[day][shift] = append(schedule[day][shift], employees[employeeIndex].Name)
						employees[employeeIndex].DaysWorked++
						employees[employeeIndex].WorkedDays[day] = true
						assigned = true
						break
					}
				}

				if !assigned {
					fmt.Println("Warning: Not enough employees available for", day, shift)
					break
				}
			}

			if len(schedule[day][shift]) > 2 {
				extraEmployee := schedule[day][shift][len(schedule[day][shift])-1]
				schedule[day][shift] = schedule[day][shift][:len(schedule[day][shift])-1]

				resolved := false

				for s := shiftIndex + 1; s < len(shifts); s++ {
					if len(schedule[day][shifts[s]]) < 2 {
						schedule[day][shifts[s]] = append(schedule[day][shifts[s]], extraEmployee)
						resolved = true
						break
					}
				}

				if !resolved && dayIndex+1 < len(days) {
					nextDay := days[dayIndex+1]

					for _, nextShift := range shifts {
						if len(schedule[nextDay][nextShift]) < 2 {
							schedule[nextDay][nextShift] = append(schedule[nextDay][nextShift], extraEmployee)
							resolved = true
							break
						}
					}
				}
			}
		}
	}

	fmt.Println("FINAL WEEKLY EMPLOYEE SCHEDULE")
	fmt.Println("================================")

	for _, day := range days {
		fmt.Println("\n" + day)
		fmt.Println("--------------------")

		for _, shift := range shifts {
			fmt.Printf("%s Shift: ", shift)

			for i, employeeName := range schedule[day][shift] {
				if i > 0 {
					fmt.Print(", ")
				}
				fmt.Print(employeeName)
			}

			fmt.Println()
		}
	}

	fmt.Println("\nEMPLOYEE WORKLOAD SUMMARY")
	fmt.Println("==========================")

	for _, employee := range employees {
		fmt.Printf("%s worked %d day(s)\n", employee.Name, employee.DaysWorked)
	}
}
