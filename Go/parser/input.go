package parser

import (
	"fmt"
	"log"
	"slices"
	"strings"
	"os"
	"bufio"
	"strconv"
	"regexp"
)

func Validate(input []string) ([]string, error) {
	valid := []string{"+", "-", "*", "/", "(", ")", "^"}
	
	var result []string

	for i, s := range input {
		_, nErr := strconv.Atoi(s)
		if !(slices.Contains(valid, s) || nErr == nil || s == "x") {
			return input, fmt.Errorf("Invalid symbol: %s", s)
		}

		if i+1 < len(input) {
			if s == input[i+1] {
				if s == "(" || s == ")" || nErr == nil {
				} else {
					return input, fmt.Errorf("repeated Symbol: %s", s)
				}
			}
		}

		if s == "x" {
			if i > 0 {
				prev := input[i-1]
				_, nErr = strconv.Atoi(prev)
				if nErr == nil || prev == ")" {
					result = append(result, "*")
				}
			}

			result = append(result, s)

			if i+1 < len(input) {
				next := input[i+1]
				_, nErr = strconv.Atoi(next)
				if nErr == nil || next == "(" {
					result = append(result, "*")
				}
			}
		} else {
			result = append(result, s)
		}
	}

	return result, nil
}

func GetRawInput() string {
	var input string

	fmt.Println("Welcome to my Graph Plotter, please provide a function.")
	fmt.Println("Valid Operators are currently: '+', '-', '*', '/', '^', '(', ')'")
	fmt.Print("f(x) = ")
	
	in := bufio.NewReader(os.Stdin)
	input, err := in.ReadString('\n')
	if err != nil {
		log.Fatalf("failed to read input: %s", err)
	}

	input = strings.ReplaceAll(input, " ", "")
	input = strings.TrimSpace(input)
	
	return input
}

func Split(input string) []string {
	var result []string

	re := regexp.MustCompile(`(\d+|[+\-*/^()x])`)
	result = re.FindAllString(input, -1)

	return result

}
