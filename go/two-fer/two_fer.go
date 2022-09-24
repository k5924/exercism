package twofer

// Package fmt allows me to format strings
// using Sprintf.
import "fmt"

// ShareWith returns One for you and one for me
// but if a name is provided, name replaces you.
func ShareWith(name string) string {
	if (name!=""){
        return fmt.Sprintf("One for %s, one for me.", name)
    }
	return fmt.Sprintf("One for you, one for me.")
}
