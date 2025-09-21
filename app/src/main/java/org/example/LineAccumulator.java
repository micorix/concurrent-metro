package org.example;

import java.util.ArrayList;
import java.util.List;
import java.util.Deque;
import java.util.LinkedList;

class LineAccumulator {
    enum CoordsInLineResult {
        STRAIGHT,
        INCONCLUSIVE,
        NOT_STRAIGHT
    }
    private final Deque<Coords> coordsStack = new LinkedList<>();

    private int lineAxis = -1;
    private int lineAxisCoordValue = -1;

    public static LineAccumulator fromPathIterator(PathIterator pathIterator) {
        LineAccumulator line = new LineAccumulator();
        int i = 0;
        while(true) {
            LineAccumulator.CoordsInLineResult result = line.addCoords(pathIterator.peek(i).coords());
            if(result == LineAccumulator.CoordsInLineResult.NOT_STRAIGHT) {
                break;
            }
            i++;
        }
        return line;
    }

    public CoordsInLineResult addCoords(Coords coords) {
        if(coordsStack.isEmpty()) {
            coordsStack.addFirst(coords);
            return CoordsInLineResult.INCONCLUSIVE;
        }

        if(canBeInStraightLine(coordsStack.getFirst(), coords)) {
            coordsStack.addFirst(coords);
            return CoordsInLineResult.STRAIGHT;
        } else {
            return CoordsInLineResult.NOT_STRAIGHT;
        }
    }

    public List<Coords> collect() {
        if(coordsStack.size() <= 1) {
            return new ArrayList<Coords>();
        }
        return new ArrayList<Coords>(coordsStack);
    }

    private boolean canBeInStraightLine(Coords a, Coords b) {
        if(lineAxis == -1) {
            if(a.x() == b.x()) {
                lineAxis = 0;
                lineAxisCoordValue = a.x();
                return true;
            } else if(a.y() == b.y()) {
                lineAxis = 1;
                lineAxisCoordValue = a.y();
                return true;
            } else {
                return false;
            }
        }

        if(lineAxis == 0) {
            return a.x() == lineAxisCoordValue && b.x() == lineAxisCoordValue;
        } else {
            return a.y() == lineAxisCoordValue && b.y() == lineAxisCoordValue;
        }
    }
}