package partyrobot

import "fmt"

// Welcome greets a person by name.
func Welcome(name string) string {
    return fmt.Sprintf("Welcome to my party, %s!", name)
}

// HappyBirthday wishes happy birthday to the birthday person and exclaims their age.
func HappyBirthday(name string, age int) string {
    result := fmt.Sprintf("Happy birthday %s!", name)
    return result + fmt.Sprintf(" You are now %d years old!", age)
}

// AssignTable assigns a table to each guest.
func AssignTable(name string, table int, neighbor, direction string, distance float64) string {
    result := Welcome(name)
    result += fmt.Sprintf("\nYou have been assigned to table %03d. ", table)
    result += fmt.Sprintf("Your table is %s, ", direction)
    result += fmt.Sprintf("exactly %.1f meters from here.\n", distance)
    return result + fmt.Sprintf("You will be sitting next to %s.", neighbor)
}
