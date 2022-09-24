// Package weather provides an i3bar
// module that displays weather info.
package weather

// CurrentCondition stores the current
// weather condition as a string.
var CurrentCondition string

// CurrentLocation stores the current
// location as a string.
var CurrentLocation string

// Forecast returns a string depicting the current weather
// conditions in a location.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
