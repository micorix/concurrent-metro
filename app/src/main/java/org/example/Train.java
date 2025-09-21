package org.example;

import javafx.application.Platform;
import javafx.scene.layout.GridPane;
import javafx.scene.paint.Color;
import javafx.scene.shape.Rectangle;

import java.util.ArrayList;
import java.util.List;


public class Train implements Runnable {
    private final GridPane board;
    private final String id;
    private final Color color;
    private final Coords[] path;
    private final List<Rectangle> cars = new ArrayList<>();
    private final CoordinatesLock coordinatesLock;

    private final int MAX_CARS = 3;
    private final int SLEEP_TIME_IN_MS = 500;

    public Train(GridPane board, String id, Color color, Coords[] path, CoordinatesLock coordinatesLock) {
        this.board = board;
        this.id = id;
        this.color = color;
        this.path = path;
        this.coordinatesLock = coordinatesLock;
    }

    @Override
    public void run() {
        PathIterator pathIterator = new PathIterator(path);
        while (true) {
            while (pathIterator.hasNext()) {
                PathElement pathElement = pathIterator.next();
                PathElement nextPathElement = pathIterator.peek(1);

                ArrayList<Coords> coordsToLock = new ArrayList<>();

                List<Coords> straightElements = new LineAccumulator().fromPathIterator(pathIterator).collect();
                coordsToLock.add(pathElement.coords());
                coordsToLock.addAll(straightElements);

                try {
                    List<Coords> lockedCoords = coordinatesLock.lockCoordsList(coordsToLock);
                    System.out.println("locked " + id + " " + lockedCoords);
                } catch (InterruptedException e) {
                    e.printStackTrace();
                }

                Platform.runLater(() -> {
                    moveTrain(pathElement.coords());
                });

                try {
                    Thread.sleep(SLEEP_TIME_IN_MS);
                } catch (InterruptedException e) {
                    e.printStackTrace();
                }

                ArrayList<Coords> coordsToUnlock = new ArrayList<>();
                // last car = -1 * (MAX_CARS - 1)
                coordsToUnlock.add(pathIterator.peek(-1 * (MAX_CARS - 1)).coords());
                coordsToUnlock.addAll(straightElements);
                coordinatesLock.unlockCoordsList(coordsToUnlock);
            }
        }
    }

    private void moveTrain(Coords coords) {
        if (cars.size() == MAX_CARS) {
            Rectangle oldCar = cars.removeFirst();
            for (Rectangle car : cars) {
                car.setFill(Color.LIGHTGRAY);
            }

            board.getChildren().remove(oldCar);
        }

        Rectangle rect = new Rectangle(30, 30, color);
        board.add(rect, coords.y(), coords.x());
        cars.add(rect);
    }
}
