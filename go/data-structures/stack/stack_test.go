package main

import "testing"

func TestPopEmptyStack(t *testing.T) {
	stack := NewStack[int]()

	_, err := stack.pop()
	if err == nil {
		t.Fatal("expected error and got ok")
	}

	expectedError := "Can't pop from an empty stack"
	if err.Error() != expectedError {
		t.Fatalf("expected error %s, but received %s", expectedError, err.Error())
	}
}

func TestPopAndPush(t *testing.T) {
	stack := NewStack[int]()

	stack.push(1)
	value, _ := stack.pop()

	if value != 1 {
		t.Fatalf("Expected 1, but was %d", value)
	}
}

func TestDoublePop(t *testing.T) {
	stack := NewStack[int]()

	stack.push(1)
	stack.push(2)
	shouldBe2, _ := stack.pop()
	shouldBe1, _ := stack.pop()

	if shouldBe2 != 2 {
		t.Fatalf("Expected 2, but was %d", shouldBe2)
	}

	if shouldBe1 != 1 {
		t.Fatalf("Expected 1, but was %d", shouldBe1)
	}
}
