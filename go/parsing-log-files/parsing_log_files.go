package parsinglogfiles
import (
	"fmt"
	"regexp"
)
func IsValidLine(text string) bool {
	logRegex := regexp.MustCompile(`^((\[(ERR|TRC|DBG|INF|WRN|FTL)]).*$)`)
	return logRegex.MatchString(text)
}
func SplitLogLine(text string) []string {
	sepRegex := regexp.MustCompile(`(<[*~=-]*>)`)
	return sepRegex.Split(text, -1)
}
func CountQuotedPasswords(lines []string) int {
	passwordRegex := regexp.MustCompile(`(?i)".*(password).*"`)
	count := 0
	for _, line := range lines {
		if passwordRegex.MatchString(line) {
			count += 1
		}
	}
	return count
}
func RemoveEndOfLineText(text string) string {
	removeRegex := regexp.MustCompile(`(end-of-line\d+)`)
	return removeRegex.ReplaceAllString(text, "")
}
func TagWithUserName(lines []string) []string {
	userRegex := regexp.MustCompile(`User\s+(\S{6,})\b`)
	updated := make([]string, len(lines))
	for i, line := range lines {
		matches := userRegex.FindStringSubmatch(line)
		if matches != nil && len(matches) == 2 {
			updated[i] = fmt.Sprintf("[USR] %s %s", matches[1], line)
		} else {
			updated[i] = line
		}
	}
	return updated
}