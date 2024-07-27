package main

import (
	"math/rand"
	"testing"
)

func TestPostitiveTwoCrystalBalls(t *testing.T) {
	randomFloor := rand.Intn(10000)
	floors := make([]bool, 10000)

	for i := randomFloor; i < 10000; i++ {
		floors[i] = true
	}

	breaksAtFloor := TwoCrystalBalls(floors)
	if breaksAtFloor != randomFloor {
		t.Fatalf("Returned %d, actually breaks at %d", breaksAtFloor, randomFloor)
	}
}

func TestNegativeTwoCrystalBalls(t *testing.T) {
	floors := make([]bool, 10000)

	breaksAtFloor := TwoCrystalBalls(floors)
	if breaksAtFloor != -1 {
		t.Fatalf("Ball should not break at this height but returned floor %d", breaksAtFloor)
	}
}
