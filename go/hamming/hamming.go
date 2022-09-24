package hamming

import (
    "errors"
)

func Distance(a, b string) (int, error) {
    if len(a) != len(b) {
		return -1, errors.New("strings arent same length")
	}
	d := 0
	for i := range a {
		if a[i] != b[i] {
			d++
		}
	}
	return d, nil
}
