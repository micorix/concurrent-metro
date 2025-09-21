package org.example;

public record Coords(int x, int y) {

    boolean equals(Coords other) {
        return x == other.x && y == other.y;
    }
    @Override
    public int hashCode() {
        // https://en.wikipedia.org/wiki/Pairing_function#Cantor_pairing_function
        return (x + y) * (x + y + 1) / 2 + y;
    }
}