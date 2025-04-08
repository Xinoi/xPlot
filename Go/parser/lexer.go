package parser

import (
	"fmt"
	"slices"
	"strconv"
	"math"
)

type TokenTree struct {
	value string
	left *TokenTree
	right *TokenTree
}
func (t *TokenTree) IntoTree(rpn []string) error {
	var stack []*TokenTree

	for _, s := range rpn {
		_, nErr := strconv.Atoi(s)
		if nErr == nil || s == "x" {
			// create leaf
			stack = append(stack, &TokenTree { value: s, left:nil, right:nil })
		}else {
			rightPtr := pop(&stack)
			leftPtr := pop(&stack)
			stack = append(stack, &TokenTree{ s, leftPtr, rightPtr })
		}
	}

	if len(stack) != 1 {
		*t = TokenTree{"e", nil, nil}
		return fmt.Errorf("failed to create TokenTree")
	}else {
		*t = *stack[0]
		return nil
	}
}
func (t TokenTree) String() string {
	if t.left == nil && t.right == nil {
		return t.value
	}else  {	
		leftS := t.left.String()
		rightS := t.right.String()

		return fmt.Sprintf("[%s - ( %s ) - %s]", leftS, t.value, rightS )
	}
}
func (t *TokenTree) CalculatePoints(steps int) ([][2]float32, error) {
	var points [][2]float32

	calc_once := func(x int) error {
		filled := FillX(t, x)
		result, err := filled.Calc()
		points = append(points, [2]float32{float32(x), result})
		if err != nil {
			return err
		}	
		return nil
	}

	for i := -(steps/2); i <= steps/2; i++ {
		calc_once(i)
	} 

	return points, nil
}

func FillX(t *TokenTree, x int) *TokenTree {

	newNode := &TokenTree{value: t.value}
	
	if t.left == nil && t.right == nil {
		if t.value == "x" {
			newNode.value = strconv.Itoa(x);
		}
	}else {
		newNode.left = FillX(t.left, x)
		newNode.right = FillX(t.right, x)
	}
	return newNode
}

func (t *TokenTree) Calc() (float32, error) {

	//parameter for checking division through 0 
	const eps = 1e-6
	
		if t.left == nil && t.right == nil {
			val, err := strconv.ParseFloat(t.value, 32)
			if err != nil {
				return -1.0, fmt.Errorf("could not parse value")
			}
			return float32(val), nil
		}else {
			l, err := t.left.Calc()
			if err != nil {
				return -1.0, err 
			}
			r, err := t.right.Calc()
			if err != nil {
				return -1.0, err			}
			switch t.value {
			case "+": return l + r, nil
			case "-": return l - r, nil
			case "*": return l * r, nil
			case "/":
				if math.Abs(float64(r)) < eps {
					return -1.0, fmt.Errorf("Division through 0!")
				}else {
					return l / r, nil
				}
			case "^": return float32(math.Pow(float64(l), float64(r))), nil
			default: return 0.0, nil
			}
		}
}

func GetPrecedence(op string) int {
	switch op {
	case "+": return 1
	case "-": return 1
	case "*": return 2
	case "/": return 2
	case "^": return 3
	}
	return -1
}

func Shunting_Yard(input []string) []string {
	var queue []string
	var stack []string
	operators := []string{"+", "-", "*", "/", "^"}

	for _, s := range input {

		_, nErr := strconv.Atoi(s)
		
		if nErr == nil || s == "x" {
			queue = append(queue, s)
		}else if slices.Contains(operators, s) {
			for !(len(stack) == 0) {
				if GetPrecedence(stack[len(stack)-1]) >= GetPrecedence(s) {
					queue = append(queue, stack[len(stack)-1])
					stack = stack[:len(stack)-1]
				}else {
					break
				}
			}
			stack = append(stack, s)
		}else if s == "(" {
			stack = append(stack, s)
		}else if s == ")" {
			for stack[len(stack)-1] != "(" {
				queue = append(queue, stack[len(stack)-1])
				stack = stack[:len(stack)-1]
			}
			stack = stack[:len(stack)-1]
		}
	}
	stack = reverse(stack)
	rpn := append(queue, stack...)
	return rpn
}

func reverse[T any](slice []T) []T {
    for i := range len(slice)/2{
        slice[i], slice[len(slice)-1-i] = slice[len(slice)-1-i], slice[i]
    }
    return slice
}

func pop[T any](alist *[]T) T {
   f:=len(*alist)
   rv:=(*alist)[f-1]
   *alist=(*alist)[:f-1]
   return rv
}
