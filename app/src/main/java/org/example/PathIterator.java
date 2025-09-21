package org.example;

public class PathIterator {
    enum Direction {
        FORWARD,
        BACKWARD
    }

    private final Coords[] path;
    private Direction currentDirection;
    private int currentIndex;

    public PathIterator(Coords[] path) {
        this.path = path;
        this.currentDirection = Direction.FORWARD;
        this.currentIndex = 0;
    }

    public boolean hasNext() {
        return true;
    }

    private PathElement peek(
            int steps,
            Direction startDirection,
            boolean saveToGlobalState
    ) {
        int tmpIndex = currentIndex;
        Direction tmpDirection = startDirection;

        for (int i = 0; i < steps; i++) {
            tmpIndex = tmpIndex + (tmpDirection == Direction.FORWARD ? 1 : -1);

            if (tmpIndex >= path.length) {
                tmpIndex = path.length - 1;
                tmpDirection = Direction.BACKWARD;
            } else if (tmpIndex < 0) {
                tmpIndex = 0;
                tmpDirection = Direction.FORWARD;
            }
        }

        PathElement pathElement = new PathElement(
                tmpIndex,
                path[tmpIndex]
        );

        if (saveToGlobalState) {
            currentIndex = tmpIndex;
            currentDirection = tmpDirection;
        }

        return pathElement;
    }

    public PathElement peek(int steps) {
        Direction alternateDirection = currentDirection == Direction.FORWARD
                ? Direction.BACKWARD
                : Direction.FORWARD;
        Direction startDirection = steps >= 0 ? currentDirection : alternateDirection;

        return peek(Math.abs(steps), startDirection, false);
    }

    public PathElement next() {
        return peek(1, currentDirection, true);
    }
}

