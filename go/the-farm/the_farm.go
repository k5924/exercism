package thefarm

import (
	"errors"
	"fmt"
)

// SillyNephewError alerts when given negative cows.
type SillyNephewError struct {
	Cows int
}

func (s *SillyNephewError) Error() string {
	return fmt.Sprintf("silly nephew, there cannot be %v cows", s.Cows)
}

// DivideFood computes the fodder amount per cow for the given cows.
func DivideFood(weightFodder WeightFodder, cows int) (float64, error) {
	fodder, err := weightFodder.FodderAmount()
    if err != nil {
		if errors.Is(err, ErrScaleMalfunction)  {
            if fodder > 0 {
				return fodder * 2 / float64(cows), nil
            } else if fodder < 0 {
            	return 0, errors.New("negative fodder")
            }
        }
        return 0, err
    }
    if fodder < 0 {
        return 0, errors.New("negative fodder")
    }
    if cows == 0 {
		return 0, errors.New("division by zero")
    }
    if cows < 0 {
        return 0, &SillyNephewError{
            Cows: cows,
        }
    }
    return fodder / float64(cows), nil
}
