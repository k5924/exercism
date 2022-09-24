package birdwatcher

import "fmt"

// TotalBirdCount return the total bird count by summing
// the individual day's counts.
func TotalBirdCount(birdsPerDay []int) int {
    birdCount:=0
    for i:=0; i<len(birdsPerDay); i++{
        birdCount+=birdsPerDay[i]
    }
	return birdCount
}

// BirdsInWeek returns the total bird count by summing
// only the items belonging to the given week.
func BirdsInWeek(birdsPerDay []int, week int) int {
    birdCount:=0
    for i:=((7*week)-7); i<7*week; i++{
        birdCount+=birdsPerDay[i]
    }
	fmt.Println(birdCount)
	return birdCount
}

// FixBirdCountLog returns the bird counts after correcting
// the bird counts for alternate days.
func FixBirdCountLog(birdsPerDay []int) []int {
	for i:=0; i<len(birdsPerDay); i++{
        if ((i==0) || (i%2==0)){
            birdsPerDay[i] += 1
        }
    }
	return birdsPerDay
}
